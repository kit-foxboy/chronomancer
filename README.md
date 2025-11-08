# Chronomancer

An applet for managing system timers. Reminders, countdowns, sleep timer overrides, and even automated script execution all from one applet.

## Roadmap
- [ ] Basic timer functionality (countdown, reminder)
- [ ] System tray applet integration
- [ ] Persistent storage of timers
- [ ] Notifications on timer completion
- [ ] Sleep timer override functionality
- [ ] Localization support
- [ ] Recurring timers
- [ ] Systemd service integration

## Installation

A [justfile](./justfile) is included by default for the [casey/just][just] command runner.

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP

## Translators

[Fluent][fluent] is used for localization of the software. Fluent's translation files are found in the [i18n directory](./i18n). New translations may copy the [English (en) localization](./i18n/en) of the project, rename `en` to the desired [ISO 639-1 language code][iso-codes], and then translations can be provided for each [message identifier][fluent-guide]. If no translation is necessary, the message may be omitted.

## Packaging

If packaging for a Linux distribution, vendor dependencies locally with the `vendor` rule, and build with the vendored sources using the `build-vendored` rule. When installing files, use the `rootdir` and `prefix` variables to change installation paths.

```sh
just vendor
just build-vendored
just rootdir=debian/chronomancer prefix=/usr install
```

It is recommended to build a source tarball with the vendored dependencies, which can typically be done by running `just vendor` on the host system before it enters the build environment.

## Developers

Developers should install [rustup][rustup] and configure their editor to use [rust-analyzer][rust-analyzer]. To improve compilation times, disable LTO in the release profile, install the [mold][mold] linker, and configure [sccache][sccache] for use with Rust. The [mold][mold] linker will only improve link times if LTO is disabled.

[fluent]: https://projectfluent.org/
[fluent-guide]: https://projectfluent.org/fluent/guide/hello.html
[iso-codes]: https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
[just]: https://github.com/casey/just
[rustup]: https://rustup.rs/
[rust-analyzer]: https://rust-analyzer.github.io/
[mold]: https://github.com/rui314/mold
[sccache]: https://github.com/mozilla/sccache

## Important Note
Agentic AI has been used to generate document templates and rapidly prototype design patterns. Cosmic is still extremely new, and there aren't strong opinions on best practices and patterns yet, so this can and will evolve over time. This documentation serves more as a journal of the learning process than definitive documentation or standards. No generated code is used in production and only serves as high level examples of possible approaches. I'm against outsourcing critical thinking but I do see the value in using AI to help brainstorm and explore ideas rapidly. I find a rubber duck that talks back and writes notes and snippets of patterns I've whiteboarded to be super useful tbh. On the off chance you're a programmer reading about my dumb little project, don't be demoralized that AI is everywhere now. Remember that you're in charge and AI still makes shit up all the time. Hang in there. Just because knowing a language isn't enough to be competitive in the job market doesn't mean that you don't have a role. It's up to you to actually KNOW how things work and be able to maintain them. I've always felt technology is always supposed to make life better for humans and in my own microscopic way, I want to contribute to that. Leave the tedium to skynet and don't give up on the world of computing or yourself.

"Don't be a tech bro, the world has enough of that. What we need now is for the honest to God nerds to make loving technology respectable, ethical, and worthwhile again." 
- Kit

## Credit
Hourglass and eye icons by Robbie Pearce from the [Softies](https://www.robbiepearce.com/softies/) icon set, used under the [Creative Commons Attribution 3.0 License](https://creativecommons.org/licenses/by/3.0/).
