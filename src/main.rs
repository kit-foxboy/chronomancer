// SPDX-License-Identifier: MIT
use cosmic::{
    app::Settings,
    applet,
    iced::{Limits, Result},
};
use i18n::init;
use i18n_embed::DesktopLanguageRequester;

mod app;
mod app_messages;
mod components;
mod config;
mod i18n;
mod models;
mod pages;
mod utils;

fn main() -> Result {
    // Get the system's preferred languages.
    let requested_languages = DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    // Pretty sure this is set in the main panel rather than here?
    let _settings =
        Settings::default().size_limits(Limits::NONE.min_width(360.0).min_height(180.0));

    // Starts the application's event loop with `()` as the application's flags.
    applet::run::<app::AppModel>(())
}
