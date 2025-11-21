#!/usr/bin/env bash
#
# Flatpak Build Script for Chronomancer
#
# This script builds the Flatpak outside of VSCode's extensions.
# It handles the full build process from source to installable Flatpak.
#
# Usage:
#   ./scripts/build-flatpak.sh [OPTIONS]
#
# Options:
#   --clean         Remove build directory before building
#   --install       Install after successful build
#   --run           Run the app after install
#   --test          Test build (install to local user only)
#   --repo PATH     Custom repo path (default: .flatpak/repo)
#   --help          Show this help message
#
set -euo pipefail

APP_ID="com.vulpineinteractive.chronomancer"
MANIFEST="${1:-flatpak/${APP_ID}.yml}"
BUILD_DIR="build-dir"
REPO_DIR=".flatpak/repo"

# Shift manifest arg if provided
if [[ $# -gt 0 ]] && [[ ! "$1" =~ ^-- ]]; then
  shift
fi

# Colors
COLOR_RED=$'\033[31m'
COLOR_GREEN=$'\033[32m'
COLOR_YELLOW=$'\033[33m'
COLOR_BLUE=$'\033[34m'
COLOR_RESET=$'\033[0m'

log()  { printf "%s[BUILD]%s %s\n" "${COLOR_BLUE}" "${COLOR_RESET}" "$*"; }
info() { printf "%s[INFO]%s %s\n" "${COLOR_GREEN}" "${COLOR_RESET}" "$*"; }
warn() { printf "%s[WARN]%s %s\n" "${COLOR_YELLOW}" "${COLOR_RESET}" "$*"; }
err()  { printf "%s[ERROR]%s %s\n" "${COLOR_RED}" "${COLOR_RESET}" "$*" >&2; }

usage() {
  cat <<EOF
Flatpak Build Script for Chronomancer

Usage: $0 [MANIFEST] [OPTIONS]

Arguments:
  MANIFEST        Path to manifest file (default: flatpak/${APP_ID}.yml)

Options:
  --clean         Remove build directory before building
  --install       Install after successful build
  --run           Run the app after install
  --test          Quick test build (local user install)
  --repo PATH     Custom repo path (default: ${REPO_DIR})
  --help          Show this help message

Examples:
  $0 --clean --test                        # Clean build and test install
  $0 flatpak/custom.yml --install --run    # Use custom manifest
  $0 --clean                               # Just build (no install)

EOF
}

# Parse arguments
DO_CLEAN=0
DO_INSTALL=0
DO_RUN=0
DO_TEST=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --clean)
      DO_CLEAN=1
      shift
      ;;
    --install)
      DO_INSTALL=1
      shift
      ;;
    --run)
      DO_RUN=1
      DO_INSTALL=1  # Must install to run
      shift
      ;;
    --test)
      DO_TEST=1
      DO_INSTALL=1
      shift
      ;;
    --repo)
      REPO_DIR="$2"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      err "Unknown option: $1"
      usage
      exit 1
      ;;
  esac
done

# Check dependencies
check_deps() {
  local missing=()

  for cmd in flatpak flatpak-builder; do
    if ! command -v "$cmd" >/dev/null 2>&1; then
      missing+=("$cmd")
    fi
  done

  if [[ ${#missing[@]} -gt 0 ]]; then
    err "Missing required commands: ${missing[*]}"
    err "Install with: sudo pacman -S flatpak flatpak-builder"
    exit 1
  fi
}

# Validate files exist
validate_files() {
  if [[ ! -f "$MANIFEST" ]]; then
    err "Manifest not found: $MANIFEST"
    exit 1
  fi

  if [[ ! -f "Cargo.toml" ]]; then
    err "Cargo.toml not found. Are you in the project root?"
    exit 1
  fi

  if [[ ! -f "flatpak/cargo-sources.json" ]]; then
    warn "cargo-sources.json not found. You may need to run:"
    warn "  flatpak-cargo-generator Cargo.lock -o flatpak/cargo-sources.json"
    warn ""
    warn "Attempting to generate it now..."

    if command -v flatpak-cargo-generator >/dev/null 2>&1; then
      flatpak-cargo-generator Cargo.lock -o flatpak/cargo-sources.json
      info "Generated cargo-sources.json"
    else
      err "flatpak-cargo-generator not found."
      err "Install with: pip install --user flatpak-cargo-generator"
      err "Or: pipx install flatpak-cargo-generator"
      exit 1
    fi
  fi
}

# Clean build directory
clean_build() {
  if [[ -d "$BUILD_DIR" ]]; then
    log "Cleaning build directory: $BUILD_DIR"
    rm -rf "$BUILD_DIR"
  fi
}

# Build Flatpak
build_flatpak() {
  log "Starting Flatpak build..."
  log "Manifest: $MANIFEST"
  log "Build dir: $BUILD_DIR"
  log "Repo: $REPO_DIR"

  local build_args=(
    "--repo=$REPO_DIR"
    "--force-clean"
  )

  if [[ $DO_TEST -eq 1 ]]; then
    build_args+=("--user")
    info "Test mode: Building for local user"
  fi

  # Create repo directory if it doesn't exist
  mkdir -p "$REPO_DIR"

  # Run flatpak-builder
  info "Running flatpak-builder..."
  if flatpak-builder "${build_args[@]}" "$BUILD_DIR" "$MANIFEST"; then
    info "Build completed successfully!"
  else
    err "Build failed!"
    exit 1
  fi
}

# Install Flatpak
install_flatpak() {
  log "Installing Flatpak..."

  local repo_name="chronomancer-local"

  # Add or update local repo as a remote
  if flatpak remote-list --user | grep -q "^${repo_name}"; then
    info "Updating existing remote '${repo_name}'..."
    flatpak remote-modify --user "${repo_name}" --url="file://$(realpath ${REPO_DIR})" || true
  else
    info "Adding local repo as remote '${repo_name}'..."
    flatpak remote-add --user --no-gpg-verify "${repo_name}" "file://$(realpath ${REPO_DIR})"
  fi

  local install_args=(
    "install"
    "--user"
    "--assumeyes"
  )

  if [[ $DO_TEST -eq 1 ]]; then
    install_args+=("--reinstall")
  fi

  install_args+=("${repo_name}" "$APP_ID")

  if flatpak "${install_args[@]}"; then
    info "Installation successful!"
  else
    err "Installation failed!"
    exit 1
  fi
}

# Run the app
run_flatpak() {
  log "Running $APP_ID..."
  flatpak run "$APP_ID"
}

# Main execution
main() {
  log "Chronomancer Flatpak Builder"
  echo

  check_deps
  validate_files

  if [[ $DO_CLEAN -eq 1 ]]; then
    clean_build
  fi

  build_flatpak

  if [[ $DO_INSTALL -eq 1 ]]; then
    install_flatpak
  fi

  if [[ $DO_RUN -eq 1 ]]; then
    run_flatpak
  fi

  echo
  info "All done!"

  if [[ $DO_INSTALL -eq 0 ]]; then
    echo
    info "To install, run:"
    info "  flatpak install --user $REPO_DIR $APP_ID"
    info "Or re-run with: $0 --install"
  fi
}

main "$@"
