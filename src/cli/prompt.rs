// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate text_io;

use std::io;
use std::io::Write;

use crate::core::utils;

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn confirm(message: &str) -> bool {
    let re_yes = utils::construct_regex(r"^(1|y|yes)$");
    let re_no = utils::construct_regex(r"^(0|n|no)$");
    let response = user_input(message, |text| {
        re_yes.is_match(text) || re_no.is_match(text)
    });
    return re_yes.is_match(&response);
}

pub fn user_input<F: Fn(&String) -> bool>(message: &str, validator: F) -> String {
    loop {
        print!("{}", message);
        io::stdout().flush().expect("Could not write to console");
        // io::stdin().read_line(&mut line).expect("Error: unable to read user input!");
        let line = text_io::read!("{}\n");
        if validator(&line) {
            return line;
        }
    }
}
