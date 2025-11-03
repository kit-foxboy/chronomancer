// SPDX-License-Identifier: MIT
use cosmic::{applet, app::Settings, iced::{Limits, Result}};
use i18n_embed::DesktopLanguageRequester;
use i18n::init;

mod app;
mod config;
mod i18n;
mod components;
mod utils;
mod models;

fn main() -> Result {
    // Get the system's preferred languages.
    let requested_languages = DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    // Pretty sure this is set in the main panel rather than here?
    let _settings = Settings::default().size_limits(
        Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );

    // Starts the application's event loop with `()` as the application's flags.
    applet::run::<app::AppModel>(())
}
