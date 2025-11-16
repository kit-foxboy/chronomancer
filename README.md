# Chronomancer

An applet for managing system timers. Reminders, countdowns, sleep timer overrides, and even automated script execution all from one applet.

## Roadmap
- [x] Basic timer functionality (countdown, reminder)
- [x] System tray applet integration
- [x] Persistent storage of timers
- [x] Notifications on timer completion
- [x] Sleep timer override functionality
- [ ] Power management at set times (suspend, hibernate, shutdown, logout)
- [ ] Recurring timers
- [ ] Reminders with custom messages
- [ ] Script execution on timer completion (definitely a version 0.2 feature due to security implications)
- [ ] Systemd service integration vs using a live channel from the applet
- [ ] Localization support

## Installation

A [justfile](./justfile) is included by default for the [casey/just][just] command runner. Once releases are available, I'll add a link to the cosmic store. If building from source, the following `just` commands are available:

- `just` builds the application with the default `just build-release` recipe
- `just run` builds and runs the application
- `just install` installs the project into the system
- `just vendor` creates a vendored tarball
- `just build-vendored` compiles with vendored dependencies from that tarball
- `just check` runs clippy on the project to check for linter warnings
- `just check-json` can be used by IDEs that support LSP

Note that just install will need to be done with elevated privileges to write to system directories. Cosmic doesn't currently support user-level applets outside of flatpaks on the cosmic store as best as I can tell, so installation to system directories is necessary.

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

## Contributing
Contributions are more than welcome! Please open issues for bug reports or feature requests of any kind. Pull requests are also encouraged for bug fixes, improvements, or new features. Suggestions for better design patterns and architecture are especially appreciated as I'm still learning Cosmic and Rust. Part of why I admittedly overdesigned this app is to explore standards in the cosmic app community and I wanted to have at least the beginnings of reusable and scalable components. Feel free to reach out or revise my guides with some better rust wisdom for my poor JavaScript-addled tiny peanut brain XwX

## Contact Information
* Always subject to my day job and health concerns as autoimmune disease doesn't operate on a predictable schedule

- Discord (most preferred): kitkabbit
- Email (if you really must): foxykit@gmail.com
- Twitch (livestreams playing games and talking about coding, game dev, and tech with a diverse group of wacky people who enjoy socializing and being dorks): [https://twitch.tv/teamsnowdog](https://twitch.tv/teamsnowdog)
- Youtube (justrecorded gaming livestreams at the moment, but more dev-centric video essays and breakdowns in the works): [https://youtube.com/@teamsnowdog](https://youtube.com/@teamsnowdog)
- Ko-fi (support my work and ideas as I try to push through medical struggles and get doodles of my ideas and little articles about my experiences and opinions on all things software. Custom software comms in the works if ever healthy enough): [https://ko-fi.com/kitkabbit4209](https://ko-fi.com/kitkabbit)
- Bluesky: I'd list that but tbh it's where I'm most openly furry trash and not super relevant to my dev work so I'll leave that one out for now. Ask if you really want it.

## Financial Support

If you find this project useful and would like to support my further involvement in cosmic, consider dropping me a tip on Ko-fi: [https://ko-fi.com/kitkabbit](https://ko-fi.com/kitkabbit). My health is really poor and making ends meet is a struggle, so any support means I can be more active in developing apps and livestreaming about games, coding, and development.

## Important Note / Rant
Agentic AI has been used to generate document templates and rapidly prototype design patterns in the .github folder. Cosmic is still extremely new, and there aren't strong opinions on best practices and patterns outside of MVU yet, so this can and will evolve over time. This documentation serves more as a journal of my learning process and design decisions with AI summarizing the choices made. Only rough structural code output by AI is used in production and is only meant to serve as high level examples of possible approaches. I'm against outsourcing critical thinking but I do see the value in using AI to help brainstorm and explore ideas rapidly. I find a rubber duck that talks back and writes notes and snippets of patterns I've whiteboarded to be super useful tbh. 

On the off chance you're a programmer reading about my dumb little project, don't be demoralized that AI is everywhere now. Remember that you're in charge and AI still makes shit up all the time. Hang in there. Just because knowing a language isn't enough to be competitive in the job market anymore doesn't mean that you don't have a role. It's up to you to actually KNOW how things work and be able to maintain them. I've always felt technology is always supposed to make life better for humans and in my own microscopic way, I want to contribute to that. Leave the tedium to skynet and don't give up on the world of computing or yourself.

"Don't be a tech bro, the world has enough of that. What we need now is for the honest to God nerds to make loving technology respectable, ethical, and worthwhile again." 
- Kit

## Credit
Hourglass and eye icons by Robbie Pearce from the [Softies](https://www.robbiepearce.com/softies/) icon set, used under the [Creative Commons Attribution 3.0 License](https://creativecommons.org/licenses/by/3.0/).
