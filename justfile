# Chronomancer justfile - Simple build recipes

appid := 'io.vulpapps.Chronomancer'

# Build release binary
default:
    cargo build --release

# Run application with backtrace
run *args:
    RUST_BACKTRACE=1 cargo run --release {{ args }}

# Run clippy
check:
    cargo clippy --all-targets --all-features -- -W clippy::pedantic -D warnings

# Format code
fmt:
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean

# Install to system (requires root)
install:
    install -Dm755 target/release/chronomancer /usr/bin/chronomancer
    install -Dm644 resources/{{ appid }}.desktop /usr/share/applications/{{ appid }}.desktop
    install -Dm644 resources/{{ appid }}.metainfo.xml /usr/share/metainfo/{{ appid }}.metainfo.xml
    install -Dm644 resources/icons/hicolor/scalable/apps/hourglass.svg /usr/share/icons/hicolor/scalable/apps/{{ appid }}.svg
    install -Dm644 resources/icons/hicolor/scalable/apps/stay-awake.svg /usr/share/icons/hicolor/scalable/apps/{{ appid }}-stay-awake.svg

# Uninstall from system (requires root)
uninstall:
    rm -f /usr/bin/chronomancer
    rm -f /usr/share/applications/{{ appid }}.desktop
    rm -f /usr/share/metainfo/{{ appid }}.metainfo.xml
    rm -f /usr/share/icons/hicolor/scalable/apps/{{ appid }}.svg
    rm -f /usr/share/icons/hicolor/scalable/apps/{{ appid }}-eye.svg
    rm -f /usr/share/icons/hicolor/scalable/apps/{{ appid }}-stay-awake.svg

# Generate cargo-sources.json for Flatpak
flatpak-sources:
    flatpak-cargo-generator Cargo.lock -o flatpak/cargo-sources.json
