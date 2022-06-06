// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;
extern crate closure;

use self::dyn_fmt::AsStrFormatExt;

use crate::core::utils;
use crate::models::config::ConfigParams;

// ----------------------------------------------------------------
// Title screens
// ----------------------------------------------------------------

pub fn show_start_screen(config: &ConfigParams) {
    println!("");
    println!("{}", utils::dedent_ignore_first_last(
        "
        * --------------------------------
        | \x1b[92;1m{}\x1b[0m
        |
        |   version: \x1b[1m{}\x1b[0m
        |   url:     \x1b[2;4m{}\x1b[0m
        * ----------------
        {}
        "
    ).format(&[
        &config.title.to_uppercase(),
        &config.version,
        &config.url,
        "",
    ]));

    if !(&config.notes == "") {
        println!("\x1b[2mNOTE: {}\x1b[0m", &config.notes);
    }
}

pub fn show_end_screen(config: &ConfigParams) {
    println!("");
    println!("{}", utils::dedent_ignore_first_last(
        "
        Thank you for using

        * --------------------------------
        | \x1b[92;1m{}\x1b[0m
        |
        |   version: \x1b[1m{}\x1b[0m
        |   url:     \x1b[2;4m{}\x1b[0m
        * ----------------

        Terminating programme.
        {}
        "
    ).format(&[
        &config.title.to_uppercase(),
        &config.version,
        &config.url,
        "",
    ]));
}
