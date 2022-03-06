// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;
extern crate closure;

use self::dyn_fmt::AsStrFormatExt;

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

// ----------------------------------------------------------------
// Main menu
// ----------------------------------------------------------------

pub fn main_menu(config: &ConfigParams, words: &Vec<String>) {
    let mut state: WordlState;
    // first restrict list of words to appropriate size:
    let mut words_remaining = words.clone()
        .iter()
        .filter(|&word| (word.len() == config.size_of_wordle))
        .map(|word| (word.clone()))
        .collect::<Vec<String>>();

    // Main cycle:
    while words_remaining.len() > 1 {
        // sort word list by best guesses:
        let mut sorted_by_best = false;
        if words_remaining.len() <= config.max_length_for_best_optimisation {
            // tactics::advanced::reduce_sort_by_remaining_size_then_entropy_then_uniqueness(&mut words_remaining);
            tactics::advanced::reduce_sort_by_distance_then_entropy_then_uniqueness(&mut words_remaining);
        } else {
            tactics::basic::reduce_sort_by_entropy_then_uniqueness(&mut words_remaining);
            sorted_by_best = false;
        }

        // display best guesses:
        let n_remaining = words_remaining.len();
        let words_unique: Vec<String>;
        let mut suggestion = words_remaining.get(0).map(|word| word.clone());
        if sorted_by_best || n_remaining <= config.max_display_length {
            display_words(&words_remaining, n_remaining, config.max_display_length);
        } else {
            words_unique = tactics::basic::reduce_to_words_with_unique_letters(&words_remaining);
            if words_unique.len() > 0 {
                display_words(&words_unique, n_remaining, config.max_display_length);
                suggestion = words_unique.get(0).map(|word| word.clone());
            } else {
                display_words(&words_remaining, n_remaining, config.max_display_length);
            }
        }

        // ask for next guess + feedback from game:
        loop {
            let (state_, cancel, quit) = sub_menu_next_guess(config, &suggestion);
            state = state_;
            if quit {
                return;
            } else if cancel {
                continue;
            }
            break;
        }
        println!("\nThe current state is: {}.", state.to_string_with_feedback());
        // update state:
        words_remaining = state.constrain(&words_remaining);
    }

    // Handle final state:
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
        },
        None => {
            eprintln!("[\x1b[93;1mWARNING\x1b[0m] No solution found, as there are no words remaining!");
        },
    }

    // Prompt to try again:
    println!("");
    let response = cli::prompt::confirm("Would you like to try again? (y/n) >> ");
    if response.cancel || response.quit {
        return;
    } else if response.state {
        main_menu(config, words);
    }
}

fn sub_menu_next_guess(config: &ConfigParams, suggestion: &Option<String>) -> (WordlState, bool, bool) {
    // let example: WordlState = WordlState::new(EXAMPLE_GUESS, EXAMPLE_FEEDBACK);

    // give user option to select top guess:
    let response = cli::prompt::confirm("Choose the top suggestion as your next guess? (y/n) >> ");
    if response.cancel || response.quit {
        return (WordlState::empty(), response.cancel, response.quit);
    }
    let guess: String;
    if response.state {
        match suggestion {
            Some(word) => {
                guess = word.clone();
            },
            // this should not happen!
            None => {
                panic!("This should not happen!");
            },
        }
    // otherwise ask for input:
    } else {
        let response = cli::prompt::input(
            "\nEnter your guess >> ",
            // validator:
            closure::closure!(move config, |guess: &String| {
                return guess_validators::validate_guess(guess, &config);
            })
        );
        if response.cancel || response.quit {
            return (WordlState::empty(), response.cancel, response.quit);
        }
        guess = response.state;
    }

    let message = utils::dedent_ignore_first_last(
        "

        Enter the feedback to your input \x1b[1m{}\x1b[0m
          \x1b[2mE.g. if it was     \x1b[2;1mx  x  ~  x  √\x1b[0m
          \x1b[2mthen enter \x1b[4;1mxx-x1\x1b[0m\x1b[2m.\x1b[0m

        {}"
    ).format(&[
        display_word(&guess).as_str(),
        ">> ",
    ]);

    let guess_ = guess.clone();
    let response = cli::prompt::input(
        message.as_str(),
        // validator:
        closure::closure!(move guess_, move config, |feedback: &String| {
            return guess_validators::validate_feedback(&guess_, feedback, config);
        })
    );
    if response.cancel || response.quit {
        return (WordlState::empty(), response.cancel, response.quit);
    }
    let feedback = response.state;

    return (WordlState::new(guess.as_str(), feedback.as_str()), false, false);
}
