// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use crate::core::utils;
use crate::core::console::ConsoleText;
use crate::core::console::read_terminal;

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn confirm(message: &str) -> bool {
    let re_yes = utils::construct_regex(r"^(1|y|yes)$");
    let re_no = utils::construct_regex(r"^(0|n|no)$");
    let response = user_input(message, |text| {
        re_yes.is_match(text) || re_no.is_match(text)
    });
    if response.cancel || response.quit {
        return false;
    }
    return re_yes.is_match(&response.to_string());
}

pub fn user_input<F: Fn(&String) -> bool>(message: &str, validator: F) -> ConsoleText {
    loop {
        let response = read_terminal(message);
        if response.cancel || response.quit || validator(&response.to_string()) {
            return response;
        }
    }
}
