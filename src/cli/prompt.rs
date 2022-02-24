// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use crate::core::utils;
use crate::core::console;

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn confirm(message: &str) -> bool {
    let re_yes = utils::construct_regex(r"^(1|y|yes)$");
    let re_no = utils::construct_regex(r"^(0|n|no)$");
    let response = input(message, |text| {
        re_yes.is_match(text) || re_no.is_match(text)
    });
    if response.cancel || response.quit {
        return false;
    }
    return re_yes.is_match(&response.to_string());
}

pub fn input<F>(message: &str, validator: F) -> console::ConsoleResponse
    where F: Fn(&String) -> bool
{
    loop {
        let response = console::interaction(message);
        if response.cancel || response.quit || validator(&response.to_string()) {
            return response;
        }
    }
}
