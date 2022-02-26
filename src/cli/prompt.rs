// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use crate::core::utils;
use crate::core::console;

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn confirm(message: &str) -> console::ConsoleResponse<bool> {
    let re_yes = utils::construct_regex(r"^(1|y|yes)$");
    let re_no = utils::construct_regex(r"^(0|n|no)$");
    let response = input(message, |text| {
        re_yes.is_match(text) || re_no.is_match(text) || text == ""
    });
    return console::ConsoleResponse {
        cancel: response.cancel,
        quit: response.quit,
        // interpret yes / [empty +]enter as yes:
        state: re_yes.is_match(&response.state) || response.state == "",
    };
}

pub fn input<F>(message: &str, validator: F) -> console::ConsoleResponse<String>
    where F: Fn(&String) -> bool
{
    loop {
        let response = console::interaction(message);
        if response.cancel || response.quit || validator(&response.state) {
            return response;
        }
    }
}
