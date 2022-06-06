// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;
extern crate regex;
extern crate itertools;

use self::regex::Regex;
use self::itertools::free::join;

use crate::core::utils;
use crate::cli::prompt;
use crate::models::config::ConfigParams;
use crate::models::response::ResponseState;

// ----------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------

pub static INDENT: &str = "  ";
pub static ON_ERROR: &str = "[\x1b[91;1mERROR\x1b[0m] Invalid selection!";

// ----------------------------------------------------------------
// Structures menu option/s
// ----------------------------------------------------------------

#[allow(dead_code)]
pub struct MenuOption<'life> {
    index: Option<usize>,
    display: String,
    keys: Vec::<String>,
    re: Option<Regex>,
    action: &'life dyn Fn(&mut ConfigParams) -> ResponseState,
}

#[allow(dead_code)]
pub struct MenuOptions<'life> {
    title: String,
    on_error: String,
    indent: String,
    options: Vec::<MenuOption<'life>>,
}

// ----------------------------------------------------------------
// Implementation of menu option
// ----------------------------------------------------------------

impl<'life> MenuOption<'life> {

    pub fn new(
        index: Option<usize>,
        display: String,
        keys: Vec<String>,
        pattern: Option<String>,
        action: &'life dyn Fn(&mut ConfigParams) -> ResponseState,
    ) -> Self {
        let re: Option<Regex>;
        match pattern {
            Some(p) => {
                re = Some(utils::construct_regex(&p));
            },
            None => {
                re = None;
            }
        }
        return Self {
            index: index,
            display: display,
            keys: keys,
            re: re,
            action: action,
        }
    }

    pub fn is_match(self: &Self, entry: &String) -> bool {
        match self.index {
            Some(index) => {
                if *entry == index.to_string() {
                    return true;
                }
            },
            None => {},
        }
        match &self.re {
            Some(re) => {
                return re.is_match(entry);
            },
            None => {
                for obj in self.keys.iter() {
                    let key: &String = obj;
                    if *entry == *key { return true; }
                }
            },
        }
        return false;
    }

    pub fn to_string(self: &Self) -> String {
        let mut expr = self.display.clone();
        if self.keys.len() > 0 {
            expr = format!("{} [\x1b[1m{}\x1b[0m]", expr, join(&self.keys, "/"));
        }
        match self.index {
            Some(index) => {
                expr = format!("\x1b[1m{}\x1b[0m) {}", index.to_string(), expr);
            },
            None => { },
        }
        return expr;
    }

    pub fn exec(self: &Self, config: &mut ConfigParams) -> ResponseState {
        let action = self.action;
        return action(config);
    }
}

// ----------------------------------------------------------------
// Implementation of menu options
// ----------------------------------------------------------------

impl<'life> MenuOptions<'life> {

    pub fn new(
        title: String,
        on_error: String,
        indent: String,
        options: Vec::<MenuOption<'life>>
    ) -> Self {
        return Self { title, indent, on_error, options }
    }

    pub fn item(self: &Self, index: usize) -> &MenuOption<'life> {
        match self.options.get(index) {
            Some(option) => {
                return option;
            },
            None => {
                panic!("Index out of bounds!");
            }
        }
    }

    pub fn get_match(self: &Self, entry: &String) -> Option<&MenuOption<'life>> {
        for obj in self.options.iter().enumerate() {
            let (index, option): (usize, &MenuOption<'life>) = obj;
            if option.is_match(entry) {
                return Some(self.item(index))
            }
        }
        return None;
    }

    pub fn to_string(self: &Self) -> String {
        let mut expr = self.title.clone();
        expr += "\n";
        for obj in self.options.iter().enumerate() {
            let (_, option): (usize, &MenuOption<'life>) = obj;
            expr = format!("{}\n{}{}", expr, self.indent, option.to_string());
        }
        expr += "\n\nEnter your selection >> ";
        return expr;
    }

    pub fn exec(self: &Self, config: &mut ConfigParams) -> ResponseState {
        loop {
            println!("");

            let (response, _) = prompt::input(
                self.to_string().as_str(),
                None,
                // accept all entries -> perform validation afterwards
                |_| Some(()),
            );
            // return with cancel/quit information, if meta keys detected:
            if response.state.is_cancel_or_quit() {
                return response.state;
            }
            // else perform validation
            let entry = response.value;
            match self.get_match(&entry) {
                Some(option) => {
                    // execute option if valid option chosen:
                    return option.exec(config);
                },
                // else do nothing
                None => { },
            }
            // if loop not yet broken, print error and continue
            println!("{}", self.on_error);
        }
    }

    pub fn cycle(self: &Self, config: &mut ConfigParams) -> ResponseState {
        loop {
            // carry out one cycle of the menu:
            let state = self.exec(config);
            // decide whether to return or continue:
            if state.is_cancel() {
                // use cancel only for this menu, then in higher up menus return 'completed':
                println!("cancelled -> complete");
                return ResponseState::COMPLETED;
            } else if state.is_quit() {
                // return quit all the way to the highest instance:
                println!("quit");
                return ResponseState::QUIT;
            }
            // else do nothing.
        }
    }
}
