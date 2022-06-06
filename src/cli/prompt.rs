// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

use crate::core::utils;
use crate::models::console;

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn confirm(message: &str) -> console::ConsoleResponse<bool> {
    let re_yes = utils::construct_regex(r"^(1|y|yes)$");
    let re_no = utils::construct_regex(r"^(0|n|no)$");
    let (response, _) = input(message, None, |text| {
        if re_yes.is_match(text) || re_no.is_match(text) || text == "" {
            return Some(())
        }
        return None
    });
    return console::ConsoleResponse {
        state: response.state,
        // interpret yes / [empty +]enter as yes:
        value: re_yes.is_match(&response.value) || response.value == "",
    };
}

pub fn input<F, T>(message: &str, error_message: Option<&str>, validator: F) -> (console::ConsoleResponse<String>, Option<T>)
    where F: Fn(&String) -> Option<T>
{
    loop {
        let response = console::interaction(message);
        // check if cancel/quit/valid response; if so, return state + None
        if response.state.is_cancel_or_quit() { return (response, None); }
        // check if valid; if so, return state + Some(value):
        match validator(&response.value) {
            Some(value) => { return (response, Some(value)); },
            None => {
                match error_message {
                    Some(err) => {
                        println!("{}", err);
                    },
                    None => { },
                }
            },
        }
    }
}
