// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;
extern crate closure;

use std::fs;

use crate::cli;
use crate::core::utils::read_file;
use crate::models::config::ConfigParams;
use crate::models::menu::INDENT;
use crate::models::menu::ON_ERROR;
use crate::models::menu::MenuOption;
use crate::models::menu::MenuOptions;
use crate::models::response::ResponseState;

// ----------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------

//

// ----------------------------------------------------------------
// Menu
// ----------------------------------------------------------------

pub fn menu_settings(config: &mut ConfigParams) -> ResponseState {
    let menu = MenuOptions::new(
        "Game settings".to_string(),
        ON_ERROR.to_string(),
        INDENT.to_string(),
        vec![
            MenuOption::new(
                Some(1),
                "Set word-size".to_string(),
                vec![],
                None,
                &closure::closure!(|config| {
                    return action_set_word_size(config);
                }),
            ),
            MenuOption::new(
                Some(2),
                "Choose word list from path".to_string(),
                vec![],
                None,
                &closure::closure!(|config| {
                    return action_set_word_list_from_path(config);
                }),
            ),
            MenuOption::new(
                Some(3),
                "Choose word list from preset options".to_string(),
                vec![],
                None,
                &closure::closure!(|config| {
                    return action_set_word_list_from_defaults(config);
                }),
            ),
            MenuOption::new(
                None,
                "Return".to_string(),
                vec!["r".to_string(), "q".to_string()],
                Some(r"^(r|q|quit)$".to_string()),
                &closure::closure!(|_| ResponseState::CANCEL),
            ),
        ],
    );
    return menu.cycle(config);
}

// ----------------------------------------------------------------
// Actions
// ----------------------------------------------------------------

fn action_set_word_size(config: &mut ConfigParams) -> ResponseState {
    let (response, value) = cli::prompt::input(
        "\nEnter the desired word size (integer > 0) >> ",
        Some("Entry must be an integer > 0."),
        // validator:
        |entry: &String| -> Option<usize> {
            match entry.parse::<usize>() {
                Err(_) => {
                    return None;
                },
                Ok(n) => {
                    if n > 0 {
                        return Some(n);
                    }
                    return None;
                },
            }
        },
    );
    if response.state.is_quit() {
        return response.state;
    } else if response.state.is_cancel() {
        return ResponseState::COMPLETED;
    } else {
        config.size_of_wordle = value.unwrap(); //.unwrap_or(panic!("should not happen"));
        return ResponseState::COMPLETED;
    }
}

// ----------------------------------------------------------------
// Actions
// ----------------------------------------------------------------

fn action_set_word_list_from_defaults<'life>(_config: &'life mut ConfigParams) -> ResponseState {
    // // FIXME!
    // let mut options = Vec::<MenuOption>::new();
    // let mut actions: Vec<&'life dyn Fn(&mut ConfigParams) -> ResponseState> = Vec::new();
    // for (_index, path) in config.paths.iter().enumerate() {
    //     let path_ = path.clone();
    //     let action: &dyn Fn(&mut ConfigParams) -> ResponseState = &closure::closure!(move path_, |config: &mut ConfigParams| {
    //         config.path_to_words = path_;
    //         return ResponseState::COMPLETED;
    //     });
    //     actions.push(action.borrow());
    // }
    // for (index, path) in config.paths.iter().enumerate() {
    //     let path_ = path.clone();
    //     options.push(MenuOption::new(
    //         Some(index),
    //         path.clone(),
    //         vec![],
    //         None,
    //         actions.get(index).unwrap(),
    //     ));
    // }
    // options.push(MenuOption::new(
    //     None,
    //     "Return".to_string(),
    //     vec!["r".to_string(), "q".to_string()],
    //     Some(r"^(r|q|quit)$".to_string()),
    //     &closure::closure!(|_| ResponseState::CANCEL),
    // ));
    // let menu:MenuOptions<'life> = MenuOptions::new(
    //     "Select one of the following paths:".to_string(),
    //     ON_ERROR.to_string(),
    //     INDENT.to_string(),
    //     options,
    // );
    // return menu.cycle(config);
    return ResponseState::COMPLETED;
}

// ----------------------------------------------------------------
// Actions
// ----------------------------------------------------------------

fn action_set_word_list_from_path(config: &mut ConfigParams) -> ResponseState {
    // config.path_to_words = ""
    let (response, value) = cli::prompt::input(
        "\nEnter a valid path >> ",
        Some("Invalid path or file unreadable."),
        // validator:
        |path: &String| -> Option<String> {
            let result = read_file(path);
            let metadata = fs::metadata(path);
            match (result, metadata) {
                (Ok(_), Ok(m)) => {
                    if m.is_file() {
                        return Some(path.clone());
                    }
                },
                _ => { },
            }
            return None;
        },
    );
    if response.state.is_quit() {
        return response.state;
    } else if response.state.is_cancel() {
        return ResponseState::COMPLETED;
    } else {
        config.path_to_words = value.unwrap(); //.unwrap_or(panic!("should not happen"));
        return ResponseState::COMPLETED;
    }
}
