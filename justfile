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
    cargo clippy --all-features -- -W clippy::pedantic

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
    for icon in resources/icons/hicolor/scalable/apps/{{ appid }}-*.svg; do \
        [ -f "$$icon" ] && install -Dm644 "$$icon" /usr/share/icons/hicolor/scalable/apps/$$(basename "$$icon"); \
    done

# Generate cargo-sources.json for Flatpak
flatpak-sources:
    python3 flatpak-cargo-generator Cargo.lock -o flatpak/cargo-sources.json
