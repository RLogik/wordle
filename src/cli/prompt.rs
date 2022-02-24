// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate text_io;

use std::io;
use std::io::Write;

// ----------------------------------------------------------------
// Prompt confirm
// ----------------------------------------------------------------

pub fn confirm(_message: &str) -> bool {
    return true;
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
