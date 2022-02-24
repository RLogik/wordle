// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;
extern crate closure;

use self::dyn_fmt::AsStrFormatExt;
use std::io::Error;

use crate::core::utils;
use crate::cli;
use crate::display::basic::display_word;
use crate::display::basic::display_words;
use crate::setup::config::ConfigParams;
use crate::app::validators::guess_validators;
use crate::app::states::WordlState;
use crate::app::tactics;

// ----------------------------------------------------------------
// Constants
// ----------------------------------------------------------------

#[allow(dead_code)]
static EXAMPLE_GUESS: &str = "alert";
#[allow(dead_code)]
static EXAMPLE_FEEDBACK: &str = "xx-x1";

// ----------------------------------------------------------------
// Main menu
// ----------------------------------------------------------------

pub fn main_menu(config: &ConfigParams, words: &Vec<String>) -> Result<String, Error> {
    let mut state: WordlState;
    let mut words_remaining = words.clone();
    while words_remaining.len() > 1 {
        // sort word list by best guesses:
        if words_remaining.len() <= config.max_length_for_best_optimisation as usize {
            // tactics::advanced::reduce_sort_by_remaining_size_then_entropy_then_uniqueness(&mut words_remaining);
            tactics::advanced::reduce_sort_by_distance_then_entropy_then_uniqueness(&mut words_remaining);
        } else {
            tactics::basic::reduce_sort_by_entropy_then_uniqueness(&mut words_remaining);
        }
        // display best guesses:
        let words_unique = tactics::basic::reduce_to_words_with_unique_letters(&words_remaining);
        if words_unique.len() > 0 {
            display_words(&words_unique, config.max_display_length as i32)
        } else {
            display_words(&words_remaining, config.max_display_length as i32)
        }
        // ask for next guess + feedback from game:
        state = sub_menu_next_guess(config);
        println!("{}", utils::dedent_ignore_first_last(
            "
            You entered
              {}.
            "
        ).format(&[state.to_string_with_feedback()]));
        // update state:
        words_remaining = state.constrain(&words_remaining);
    }

    println!("");
    match words_remaining.get(0) {
        Some(word) => {
            let state = WordlState::from(word, word);
            println!("{}", utils::dedent_ignore_first_last(
                "
                Only one word left. The solution must be:
                  {}.
                "
            ).format(&[state.to_string_with_feedback()]));
            println!("");
            return Ok(word.clone());
        },
        None => {
            // TODO: replace by proper error.
            panic!("Something went wrong! No words remaining!")
        },
    }
}

fn sub_menu_next_guess(config: &ConfigParams) -> WordlState {
    // let example: WordlState = WordlState::new(EXAMPLE_GUESS, EXAMPLE_FEEDBACK);
    let guess = cli::prompt::user_input(
        "\nEnter your guess: ",
        // validator:
        closure::closure!(move config, |guess: &String| {
            return guess_validators::validate_guess(guess, &config);
        })
    );

    let message = utils::dedent_ignore_first_last(
        "

        Enter the feedback: \x1b[1m{}\x1b[0m
          \x1b[2mE.g. if it was     \x1b[2;1mx  x  ~  x  √\x1b[0m
          \x1b[2mthen enter \x1b[4;1mxx-x1\x1b[0m\x1b[2m.\x1b[0m

        {}"
    ).format(&[
        display_word(&guess).as_str(),
        ">> ",
    ]);

    let guess_ = guess.clone();
    let feedback = cli::prompt::user_input(
        message.as_str(),
        // validator:
        closure::closure!(move guess_, move config, |feedback: &String| {
            return guess_validators::validate_feedback(&guess_, feedback, config);
        })
    );

    return WordlState::new(guess.as_str(), feedback.as_str());
}
