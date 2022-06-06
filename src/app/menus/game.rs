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
use crate::models::config::ConfigParams;
use crate::app::validators::guesses;
use crate::models::response::ResponseState;
use crate::models::states::WordleState;
use crate::app::tactics;

// ----------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------

#[allow(dead_code)]
static EXAMPLE_GUESS: &str = "alert";
#[allow(dead_code)]
static EXAMPLE_FEEDBACK: &str = "xx-x1";

// ----------------------------------------------------------------
// Game menu
// ----------------------------------------------------------------

pub fn menu_game(config: &ConfigParams, words: &Vec<String>) -> ResponseState {
    let mut state = WordleState::empty();
    // first restrict list of words to appropriate size:
    let mut words_remaining = words.clone()
        .iter()
        .filter(|&word| (word.len() == config.size_of_wordle))
        .map(|word| (word.clone()))
        .collect::<Vec<String>>();
    let mut summary = Vec::<String>::new();

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
            let (state_, _, s) = sub_menu_next_guess(config, &suggestion);
            // if quit shortcut was pressed, return:
            if s.is_quit() { return s; }
            // if cancel shortcut was return as completed:
            if s.is_cancel() { return ResponseState::COMPLETED; }
            // otherwise finish loop
            state = state_;
            break;
        }
        let feedback = state.to_string_with_feedback();
        let feedback_anon = state.to_string_with_feedback_anon();
        println!("\nThe current state is: {}.", feedback);
        summary.push(if config.anonymous_feedback { feedback_anon } else { feedback });
        // update state:
        words_remaining = state.constrain(&words_remaining);
    }

    // Handle final state:
    println!("");
    match words_remaining.get(0) {
        Some(word) => {
            // if last state was incorrect and an option remains, then add in missing feedback, as loop terminated
            if !state.is_correct() && words_remaining.len() > 0 {
                let state = WordleState::from(word, word);
                let feedback = state.to_string_with_feedback();
                let feedback_anon = state.to_string_with_feedback_anon();
                summary.push(if config.anonymous_feedback { feedback_anon } else { feedback });
            }
            // display summary:
            println!("\nThe solution is \x1b[1m{}\x1b[0m and your path to the solution was as follows:\n", word);
            for feedback in summary.iter() {
                println!("{}", feedback);
            }
        },
        None => {
            eprintln!("[\x1b[93;1mWARNING\x1b[0m] No solution found, as there are no words remaining!");
        },
    }

    // Prompt to try again:
    println!("");
    let response = cli::prompt::confirm("Would you like to try again? (y/n) >> ");
    if response.state.is_completed() && response.value == true {
        return menu_game(config, words);
    } else if response.state.is_quit() {
        return response.state;
    } else {
        return ResponseState::COMPLETED;
    }
}

// ----------------------------------------------------------------
// Auxiliary method to process guess
// ----------------------------------------------------------------

fn sub_menu_next_guess(config: &ConfigParams, suggestion: &Option<String>) -> (WordleState, String, ResponseState) {
    // let example: WordleState = WordleState::new(EXAMPLE_GUESS, EXAMPLE_FEEDBACK);

    // give user option to select top guess:
    let response = cli::prompt::confirm("Choose the top suggestion as your next guess? (y/n) >> ");
    if response.state.is_cancel_or_quit() {
        return (WordleState::empty(), String::from(""), response.state);
    }
    let guess: String;
    if response.value {
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
        let (response, _) = cli::prompt::input(
            "\nEnter your guess >> ",
            None,
            // validator:
            closure::closure!(move config, |guess: &String| {
                return guesses::validate_guess(guess, &config);
            })
        );
        if response.state.is_cancel_or_quit() {
            return (WordleState::empty(), String::from(""), response.state);
        }
        guess = response.value;
    }

    let message = utils::dedent_ignore_first_last(
        "

        Enter the feedback to your input \x1b[1m{}\x1b[0m
          \x1b[2mE.g. if it was                  \x1b[2;1mx  x  ~  x  √\x1b[0m
          \x1b[2mthen enter \x1b[4;1mxx-x1\x1b[0m\x1b[2m.\x1b[0m

        {}"
    ).format(&[
        display_word(&guess).as_str(),
        ">> ",
    ]);

    let guess_ = guess.clone();
    let (response, _) = cli::prompt::input(
        message.as_str(),
        None,
        // validator:
        closure::closure!(move guess_, move config, |feedback: &String| {
            return guesses::validate_feedback(&guess_, feedback, config);
        })
    );
    if response.state.is_cancel_or_quit() {
        return (WordleState::empty(), String::from(""), response.state);
    }
    let feedback = response.value;
    return (WordleState::new(guess.as_str(), feedback.as_str()), guess, response.state);
}
