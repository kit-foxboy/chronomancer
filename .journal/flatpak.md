# Flatpak Support (Future)

Chronomancer includes Flatpak manifest files for future distribution via the COSMIC Flatpak repository.

**For v1:** Use standard installation (`just`, `just run`, `sudo just install`)

## Generating Cargo Sources

When preparing for Flatpak submission, generate the cargo sources file:

```bash
# Install dependencies (one time)
pip install --user aiohttp toml

# Generate cargo sources
just flatpak-sources
```

This runs the included `flatpak/flatpak-cargo-generator.py` script to create `flatpak/cargo-sources.json`.

## COSMIC Flatpak Submission

Following the pattern of other COSMIC community applets, we maintain `cargo-sources.json` in our repo. The manifest itself will live in the cosmic-flatpak repository.

**When ready:**

1. Update to latest commit and tag a release
2. Generate cargo-sources.json: `just flatpak-sources`
3. Commit `flatpak/cargo-sources.json` to the repo
4. In cosmic-flatpak repo, create `app/io.vulpapps.Chronomancer/io.vulpapps.Chronomancer.json`
5. Reference the JSON manifest from our repo can be adapted for cosmic-flatpak
6. Test build with COSMIC tools: `just build io.vulpapps.Chronomancer`
7. Submit PR to https://github.com/pop-os/cosmic-flatpak

## Reference

See other COSMIC community applets in https://github.com/cosmic-utils/cosmic-project-collection for examples.