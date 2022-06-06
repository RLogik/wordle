// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;

use self::dyn_fmt::AsStrFormatExt;

use crate::core::utils;
use crate::models::config::ConfigParams;

// ----------------------------------------------------------------
// Validators
// ----------------------------------------------------------------

pub fn validate_guess(guess: &String, config: &ConfigParams) -> Option<()> {
    let n = config.size_of_wordle;
    let re1 = utils::construct_regex(r"^\w*$");
    let re2 = utils::construct_regex(r"^\D*$");
    if utils::length_of_word(guess) == n && re1.is_match(guess) && re2.is_match(guess) {
        return Some(());
    } else {
        println!("{}\n", utils::dedent_ignore_first_last(
            "
            [\x1b[91mERROR\x1b[0m] Invalid Guess!

            - Must consist of letters.
            - Length of guess must be {}.
            "
        ).format(&[n]));
        return None;
    }
}

pub fn validate_feedback(_guess: &String, feedback: &String, config: &ConfigParams) -> Option<()> {
    let n = config.size_of_wordle;
    let re = utils::construct_regex(r"^[01x-]*$");
    if utils::length_of_word(feedback) == n && re.is_match(feedback) {
        return Some(());
    } else {
        print!("{}\n", utils::dedent_ignore_first_last(
            "
            [\x1b[91mERROR\x1b[0m] Invalid Feedback option!

            - Format must only contain the symbols: 0, 1, -
            - Length of feedback must match length of guess.
            - Letters marked (correct|partially correct|incorrect) must be disjoint!
            "
        ).format(&[n]));
        return None;
    }
}
