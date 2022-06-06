// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;
extern crate closure;

// use self::dyn_fmt::AsStrFormatExt;

// use crate::core::utils;
// use crate::cli;
// use crate::display::basic::display_word;
// use crate::display::basic::display_words;
// use crate::app::validators::guesses;
// use crate::models::states::WordleState;
// use crate::app::tactics;
use crate::setup::assets;
use crate::models::menu::INDENT;
use crate::models::menu::ON_ERROR;
use crate::models::menu::MenuOption;
use crate::models::menu::MenuOptions;
use crate::models::response::ResponseState;
use crate::app::menus::game::menu_game;
use crate::app::menus::settings::menu_settings;

// ----------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------

//

// ----------------------------------------------------------------
// Main menu
// ----------------------------------------------------------------

pub fn create_main_menu<'life>() -> MenuOptions<'life> {
    return MenuOptions::new(
        "Main menu".to_string(),
        ON_ERROR.to_string(),
        INDENT.to_string(),
        vec![
            MenuOption::new(
                Some(1),
                "Play the game".to_string(),
                vec!["play".to_string()],
                None,
                &closure::closure!(|config| {
                    let words = assets::get_data(&config.path_to_words)
                        .unwrap_or_else(|err| panic!("{}", err));
                    return menu_game(config, &words);
                }),
            ),
            MenuOption::new(
                Some(2),
                "Adjust game settings".to_string(),
                vec!["settings".to_string()],
                None,
                &closure::closure!(|config| {
                    return menu_settings(config);
                }),
            ),
            MenuOption::new(
                None,
                "Quit".to_string(),
                vec!["q".to_string()],
                Some(r"^(q|quit)$".to_string()),
                &closure::closure!(|_| ResponseState::QUIT),
            ),
        ],
    );
}
