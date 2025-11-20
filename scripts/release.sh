#!/usr/bin/env bash
#
# Chronomancer Release Helper Script
#
# Automates tagging, cargo-sources generation, Flatpak manifest updates,
# and basic validation for a new release. This took me a hot minute to get right,
# so I'm sharing it here for future me and anyone else who might find it useful.
#
# Usage:
#   ./scripts/release.sh v0.1.0
#
# Optional environment variables:
#   DRY_RUN=1          # Show actions without modifying files
#   SKIP_TAG=1         # Do not create / push git tag
#   SKIP_SOURCES=1     # Skip cargo-sources.json generation
#   SKIP_SHA=1         # Skip downloading tarball + sha256 update
#   NO_VALIDATE=1      # Skip AppStream validation
#
# Requirements:
#   - git
#   - curl
#   - sha256sum
#   - sed
#   - pip or pipx (to install flatpak-cargo-generator if missing)
#   - appstreamcli (recommended; install via your distro)
#
# After running, manually:
#   - Commit changes: git add flatpak/${APP_ID}.yml cargo-sources.json resources/app.metainfo.xml
#   - git commit -m "Release ${VERSION}"
#   - Test Flatpak build locally before opening Flathub PR.
#
set -euo pipefail

APP_ID="com.vulpineinteractive.chronomancer"
MANIFEST="flatpak/${APP_ID}.yml"
CARGO_LOCK="Cargo.lock"
CARGO_SOURCES="cargo-sources.json"

# A little fanciness goes a long way
COLOR_RED=$'\033[31m'
COLOR_GREEN=$'\033[32m'
COLOR_YELLOW=$'\033[33m'
COLOR_BLUE=$'\033[34m'
COLOR_DIM=$'\033[2m'
COLOR_RESET=$'\033[0m'

log()  { printf "%s[%s]%s %s\n" "${COLOR_BLUE}" "$(date +'%H:%M:%S')" "${COLOR_RESET}" "$*"; }
info() { printf "%s[INFO]%s %s\n" "${COLOR_GREEN}" "${COLOR_RESET}" "$*"; }
warn() { printf "%s[WARN]%s %s\n" "${COLOR_YELLOW}" "${COLOR_RESET}" "$*"; }
err()  { printf "%s[ERR ]%s %s\n" "${COLOR_RED}" "${COLOR_RESET}" "$*" >&2; }

usage() {
  cat <<EOF
Chronomancer release script

Usage: $0 vX.Y.Z

Example:
  $0 v0.1.0

Environment flags:
  DRY_RUN=1       Do not modify files; just show planned actions
  SKIP_TAG=1      Skip git tagging
  SKIP_SOURCES=1  Skip cargo-sources.json generation
  SKIP_SHA=1      Skip archive download & sha256 replacement
  NO_VALIDATE=1   Skip AppStream validation

EOF
}

require_file() {
  local f="$1"
  [[ -f "$f" ]] || { err "Required file missing: $f"; exit 1; }
}

run() {
  if [[ "${DRY_RUN:-}" == "1" ]]; then
    echo "DRY_RUN: $*"
  else
    eval "$@"
  fi
}

confirm() {
  local prompt="$1"
  read -r -p "${prompt} [y/N] " ans
  [[ "${ans}" == "y" || "${ans}" == "Y" ]]
}

if [[ "${1:-}" == "" ]]; then
  usage
  exit 1
fi

VERSION="$1"

if ! [[ "${VERSION}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  err "Version must match pattern vX.Y.Z (got: ${VERSION})"
  exit 1
fi

# Extract numeric portion
VERSION_NO_V="${VERSION#v}"

log "Starting release for ${VERSION}"

require_file "Cargo.toml"
require_file "${CARGO_LOCK}"
require_file "${MANIFEST}"

# Check Cargo.toml version alignment
CARGO_TOML_VERSION=$(grep -E '^version\s*=' Cargo.toml | head -1 | cut -d'"' -f2)
if [[ "${CARGO_TOML_VERSION}" != "${VERSION_NO_V}" ]]; then
  warn "Cargo.toml version (${CARGO_TOML_VERSION}) does not match provided tag (${VERSION_NO_V})"
  if ! confirm "Proceed anyway?"; then
    err "Aborting due to version mismatch."
    exit 1
  fi
fi

# Ensure clean work tree (unless DRY_RUN)
if [[ "${DRY_RUN:-}" != "1" ]]; then
  if [[ -n "$(git status --porcelain)" ]]; then
    warn "Working tree is not clean."
    if ! confirm "Continue with uncommitted changes?"; then
      err "Aborting due to dirty work tree."
      exit 1
    fi
  fi
fi

# Tagging
if [[ "${SKIP_TAG:-}" == "1" ]]; then
  info "Skipping git tag creation (SKIP_TAG=1)"
else
  if git rev-parse "${VERSION}" >/dev/null 2>&1; then
    warn "Tag ${VERSION} already exists."
  else
    info "Creating git tag ${VERSION}"
    run "git tag -a '${VERSION}' -m '${VERSION}'"
    if [[ "${DRY_RUN:-}" != "1" ]]; then
      info "Pushing tag"
      run "git push --tags"
    fi
  fi
fi

# Generate cargo-sources.json
if [[ "${SKIP_SOURCES:-}" == "1" ]]; then
  info "Skipping cargo-sources generation (SKIP_SOURCES=1)"
else
  if ! command -v flatpak-cargo-generator >/dev/null 2>&1; then
    warn "flatpak-cargo-generator not found, attempting install via pipx or pip"
    if [[ "${DRY_RUN:-}" != "1" ]]; then
      if command -v pipx >/dev/null 2>&1; then
        info "Installing flatpak-cargo-generator via pipx"
        pipx install flatpak-cargo-generator || {
          err "Failed to install flatpak-cargo-generator with pipx"
          exit 1
        }
      elif command -v pip >/dev/null 2>&1; then
        info "Installing flatpak-cargo-generator via pip"
        pip install --user flatpak-cargo-generator || {
          err "Failed to install flatpak-cargo-generator with pip"
          exit 1
        }
      else
        err "Neither pipx nor pip found; cannot install flatpak-cargo-generator"
        exit 1
      fi
      export PATH="$HOME/.local/bin:$PATH"
    fi
  fi
  info "Generating ${CARGO_SOURCES}"
  run "flatpak-cargo-generator ${CARGO_LOCK} -o ${CARGO_SOURCES}"
  if [[ "${DRY_RUN:-}" != "1" ]]; then
    [[ -s "${CARGO_SOURCES}" ]] || { err "cargo-sources.json is empty or missing"; exit 1; }
  fi
fi

# Download archive & compute sha256
ARCHIVE="chronomancer-${VERSION}.tar.gz"
ARCHIVE_URL="https://github.com/kit-foxboy/chronomancer/archive/refs/tags/${VERSION}.tar.gz"

if [[ "${SKIP_SHA:-}" == "1" ]]; then
  info "Skipping archive download & sha256 update (SKIP_SHA=1)"
else
  info "Downloading source archive: ${ARCHIVE_URL}"
  run "curl -L -o '${ARCHIVE}' '${ARCHIVE_URL}'"
  if [[ "${DRY_RUN:-}" != "1" ]]; then
    require_file "${ARCHIVE}"
    SHA256=$(sha256sum "${ARCHIVE}" | awk '{print $1}')
    info "Computed sha256: ${SHA256}"

    # Update manifest: url & sha256 lines
    info "Updating manifest with new URL and sha256"
    # Replace URL line if version changed
    sed -i "s|url: https://github.com/kit-foxboy/chronomancer/archive/refs/tags/v[0-9]\+\.[0-9]\+\.[0-9]\+\.tar\.gz|url: ${ARCHIVE_URL}|" "${MANIFEST}"
    # Replace sha256 line
    sed -i "s|sha256: .*|sha256: ${SHA256}|" "${MANIFEST}"
  else
    warn "DRY_RUN: Skipping manifest modification"
  fi
fi

# AppStream validation
if [[ "${NO_VALIDATE:-}" == "1" ]]; then
  info "Skipping AppStream validation (NO_VALIDATE=1)"
else
  if command -v appstreamcli >/dev/null 2>&1; then
    info "Validating AppStream metadata"
    if [[ "${DRY_RUN:-}" != "1" ]]; then
      if ! appstreamcli validate resources/app.metainfo.xml; then
        warn "AppStream validation produced errors/warnings. Review before submitting."
      else
        info "AppStream validation passed."
      fi
    else
      warn "DRY_RUN: Not executing appstreamcli"
    fi
  else
    warn "appstreamcli not found; skipping validation."
  fi
fi

# Summary
echo
info "Release preparation complete."
echo "----------------------------------------"
echo " Version tag:       ${VERSION}"
echo " Manifest:          ${MANIFEST}"
echo " Cargo sources:     ${CARGO_SOURCES} $( [[ -f ${CARGO_SOURCES} ]] && echo '[OK]' || echo '[MISSING]' )"
echo " Archive:           ${ARCHIVE} $( [[ -f ${ARCHIVE} ]] && echo '[OK]' || echo '[SKIPPED/DRY]' )"
if [[ "${SKIP_SHA:-}" != "1" && "${DRY_RUN:-}" != "1" ]]; then
  echo " sha256:            ${SHA256}"
fi
echo " Tagging skipped:   ${SKIP_TAG:-0}"
echo " Sources skipped:   ${SKIP_SOURCES:-0}"
echo " SHA skipped:       ${SKIP_SHA:-0}"
echo " Validation skipped:${NO_VALIDATE:-0}"
echo " Dry run:           ${DRY_RUN:-0}"
echo "----------------------------------------"
echo
echo "For reference, since it helps me to have this in one place, here are next steps:"
echo "  1. Inspect git diff: git diff"
echo "  2. Commit changes:   git add ${MANIFEST} ${CARGO_SOURCES} resources/app.metainfo.xml && git commit -m 'Release ${VERSION}'"
echo "  3. Test Flatpak:     flatpak-builder --user --install --force-clean build-dir ${MANIFEST}"
echo "  4. Run app:          flatpak run ${APP_ID}"
echo "  5. Open Flathub PR with updated manifest and cargo-sources.json."
echo "  6. Hope they accept it and celebrate if so! "
info "Done."

exit 0
