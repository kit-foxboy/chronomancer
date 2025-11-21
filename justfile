name := 'chronomancer'
appid := 'com.vulpineinteractive.chronomancer'
rootdir := ''
prefix := '/usr'
base-dir := absolute_path(clean(rootdir / prefix))
bin-src := 'target' / 'release' / name
bin-dst := base-dir / 'bin' / name
desktop := appid + '.desktop'
desktop-src := 'resources' / desktop
desktop-dst := clean(rootdir / prefix) / 'share' / 'applications' / desktop
appdata := appid + '.metainfo.xml'
appdata-src := 'resources' / appdata
appdata-dst := clean(rootdir / prefix) / 'share' / 'metainfo' / appdata
icons-src := 'resources' / 'icons' / 'hicolor'
icons-dst := clean(rootdir / prefix) / 'share' / 'icons' / 'hicolor'
icon-svg-src := icons-src / 'scalable' / 'apps' / 'hourglass.svg'
icon-svg-dst := icons-dst / 'scalable' / 'apps' / appid + '.svg'

# Custom icon directory for chronomancer-specific icons

custom-icons-src := icons-src / 'scalable' / 'apps'
custom-icons-dst := icons-dst / 'scalable' / 'apps'

# Flatpak paths

flatpak-script := 'scripts' / 'build-flatpak.sh'
flatpak-manifest := 'flatpak' / appid + '.yml'
cargo-sources := 'flatpak' / 'cargo-sources.json'

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Compiles with debug profile
build-debug *args:
    cargo build {{ args }}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features {{ args }} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Apply clippy pedantic auto-fixes (modifies code)
fix-clippy:
    cargo clippy --all-features --fix --allow-dirty --allow-staged -- -W clippy::pedantic

# Format Rust code
fmt:
    cargo fmt --all

# Run the application for testing purposes
run *args:
    env RUST_BACKTRACE=full cargo run --release {{ args }}

# Generate cargo-sources.json for Flatpak from Cargo.lock
flatpak-cargo-sources:
    #!/usr/bin/env sh
    set -eu
    if ! command -v flatpak-cargo-generator >/dev/null 2>&1; then
        echo "Error: flatpak-cargo-generator not found"
        echo "Install with: pip install --user flatpak-cargo-generator"
        echo "Or: pipx install flatpak-cargo-generator"
        exit 1
    fi
    echo "Generating {{ cargo-sources }} from Cargo.lock..."
    flatpak-cargo-generator Cargo.lock -o {{ cargo-sources }}
    echo "Done!"

# Build Flatpak (wrapper around scripts/build-flatpak.sh)
flatpak-build *args:
    {{ flatpak-script }} {{ flatpak-manifest }} {{ args }}

# Build Flatpak with clean
flatpak-build-clean *args:
    {{ flatpak-script }} {{ flatpak-manifest }} --clean {{ args }}

# Quick test build: clean + generate sources + build + install locally
flatpak-test *args: flatpak-cargo-sources
    {{ flatpak-script }} {{ flatpak-manifest }} --clean --test {{ args }}

# Build and install Flatpak
flatpak-install *args: flatpak-cargo-sources
    {{ flatpak-script }} {{ flatpak-manifest }} --install {{ args }}

# Build, install, and run Flatpak
flatpak-run *args: flatpak-cargo-sources
    {{ flatpak-script }} {{ flatpak-manifest }} --run {{ args }}

# Remove Flatpak build artifacts
flatpak-clean:
    rm -rf build-dir .flatpak

# Full Flatpak cleanup (including cargo-sources.json)
flatpak-clean-all: flatpak-clean
    rm -f {{ cargo-sources }}

# Convenience alias for flatpak-build
flatpak: flatpak-build

# Installs files
install:
    #!/usr/bin/env sh
    set -eu
    install -Dm0755 {{ bin-src }} {{ bin-dst }}
    install -Dm0644 {{ desktop-src }} {{ desktop-dst }}
    install -Dm0644 {{ appdata-src }} {{ appdata-dst }}
    install -Dm0644 {{ icon-svg-src }} {{ icon-svg-dst }}
    for icon in {{ custom-icons-src }}/*.svg; do
        name="${icon##*/}"
        if [ "$name" != "hourglass.svg" ]; then
            install -Dm0644 "$icon" {{ custom-icons-dst }}/"$name"
        fi
    done

# Uninstalls installed files
uninstall:
    rm {{ bin-dst }} {{ desktop-dst }} {{ icon-svg-dst }}
    # Remove all custom app icons (prefixed with app ID)
    rm -f {{ custom-icons-dst }}/{{ appid }}-*.svg

# Vendor dependencies locally
vendor:
    #!/usr/bin/env bash
    mkdir -p .cargo
    cargo vendor --sync Cargo.toml | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    echo >> .cargo/config.toml
    echo '[env]' >> .cargo/config.toml
    if [ -n "${SOURCE_DATE_EPOCH}" ]
    then
        source_date="$(date -d "@${SOURCE_DATE_EPOCH}" "+%Y-%m-%d")"
        echo "VERGEN_GIT_COMMIT_DATE = \"${source_date}\"" >> .cargo/config.toml
    fi
    if [ -n "${SOURCE_GIT_HASH}" ]
    then
        echo "VERGEN_GIT_SHA = \"${SOURCE_GIT_HASH}\"" >> .cargo/config.toml
    fi
    tar pcf vendor.tar .cargo vendor
    rm -rf .cargo vendor

# Extracts vendored dependencies
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar
