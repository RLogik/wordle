// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate crossterm;

use std::io;
use std::io::Write; // !!! need this to perform flush !!!

use self::crossterm::execute;
use self::crossterm::cursor as CrossTermCursor;
use self::crossterm::event::Event::Key;
use self::crossterm::event::KeyEvent;
use self::crossterm::event::KeyCode;
use self::crossterm::event::KeyModifiers;
use self::crossterm::event::read as KeyPressToEvent;
use self::crossterm::style::Print as CrossTermPrint;
use self::crossterm::terminal::enable_raw_mode;
use self::crossterm::terminal::disable_raw_mode;
use super::utils;

// ----------------------------------------------------------------
// Structure
// ----------------------------------------------------------------

/// ConsoleResponse<T> - generic structure to summarise responses
///
/// ## Parts ##
///
/// - `state` - value type T
/// - `state` - boolean which says if META + D was clicked
/// - `quit` - boolean which says if META + C was clicked
pub struct ConsoleResponse<T> {
    pub state: T,
    pub cancel: bool,
    pub quit: bool,
}

/// The data type for console interactions.
///
/// ## Parts ##
///
/// ```rust
/// struct ConsoleTextState {
///     pub cancel: bool, // whether user pressed META + D
///     pub quit: bool,   // whether user pressed META + C
///     // ...
/// }
/// ```
///
/// ## Methods/Examples ##
///
/// ```rust
/// use wordle::core::console::ConsoleTextState;
/// let mut response = ConsoleTextState::new();
/// assert_eq!(response.cancel, false);
/// assert_eq!(response.quit, false);
/// response.insert("this iz");
/// response.insert(" ");
/// response.insert("a");
/// response.insert(" ");
/// response.insert("text");
/// assert_eq!(response.to_string(), "this iz a text");
/// response.move_left(7);
/// response.delete();
/// assert_eq!(response.to_string(), "this i a text");
/// response.insert("s");
/// assert_eq!(response.to_string(), "this is a text");
/// response.move_right(2);
/// response.delete();
/// response.insert("some");
/// assert_eq!(response.to_string(), "this is some text");
/// ```
pub struct ConsoleTextState {
    pub cancel: bool,
    pub quit: bool,
    symbols: Vec<String>,
    cursor: usize,
}

// ----------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------

impl ConsoleTextState {
    pub fn new() -> Self {
        let symbols: Vec<String> = Vec::<String>::new();
        let cursor: usize = 0;
        return ConsoleTextState {symbols, cursor, cancel: false, quit: false};
    }

    pub fn len(self: &Self) -> usize {
        return self.symbols.len();
    }

    pub fn to_string(self: &Self) -> String {
        return self.symbols.join("");
    }

    // pub fn as_str<'life>(self: &'life mut Self) -> &'life str {
    //     return self.to_string().as_str();
    // }

    fn segment<F: Fn(usize) -> bool>(self: &Self, filt: F) -> String {
        return self.symbols
            .iter()
            .enumerate()
            .filter(|&(i, _)| filt(i))
            .map(|(_, u)| u.clone())
            .collect::<Vec<String>>()
            .join("");
    }

    pub fn split(self: &Self) -> (String,String) {
        return (
            self.segment(|i| i < self.cursor),
            self.segment(|i| i >= self.cursor),
        );
    }

    #[allow(dead_code)]
    pub fn split_three(self: &Self) -> (String,String,String) {
        return (
            self.segment(|i| i < self.cursor),
            self.segment(|i| i == self.cursor),
            self.segment(|i| i > self.cursor),
        );
    }

    pub fn move_left(self: &mut Self, i: u32) -> bool {
        let mut ii = i.clone();
        let mut moved = false;
        while ii > 0 && self.cursor > 0 {
            self.cursor -= 1;
            moved = true;
            ii -= 1;
        }
        return moved;
    }

    pub fn move_right(self: &mut Self, i: u32) -> bool {
        let mut ii = i.clone();
        let mut moved = false;
        while ii > 0 && self.cursor < self.len() {
            self.cursor += 1;
            moved = true;
            ii -= 1;
        }
        return moved;
    }

    pub fn insert(self: &mut Self, value: &str) {
        let letters = utils::string_to_chars(&value.to_string());
        let n = letters.len();
        if self.cursor >= self.len() {
            self.cursor = self.len();
            for letter in &letters {
                self.symbols.push(letter.to_string());
            }
        } else {
            let mut symbols: Vec<String> = Vec::<String>::new();
            for (i, a) in self.symbols.iter().enumerate() {
                if i == self.cursor {
                    for letter in &letters {
                        symbols.push(letter.to_string());
                    }
                }
                symbols.push(a.clone());
            }
            self.symbols = symbols;
        }
        self.move_right(n as u32);
    }

    pub fn delete(self: &mut Self) -> bool {
        if self.len() <= 0 || self.cursor == 0 {
            return false;
        }
        self.move_left(1);
        return self.delete_ahead();
    }

    pub fn delete_ahead(self: &mut Self) -> bool {
        if self.cursor >= self.len() {
            return false;
        }
        let mut text_new: Vec<String> = Vec::<String>::new();
        for (i, value) in self.symbols.iter().enumerate() {
            if i == self.cursor { continue; }
            text_new.push(value.clone());
        }
        self.symbols = text_new;
        return true;
    }
}

// ----------------------------------------------------------------
// Method
// ----------------------------------------------------------------

/// Starts an interactive session on the terminal,
/// which allows for more exact key capture.
///
/// ## Arguments ##
///
/// - `message` - an initial string to print to the console.
///               By default the cursor starts after this.
///
/// ## Returns ##
///
/// `response: ConsoleTextState`, which contains information
/// about the text entered, and whether the user has entered
/// key combinations for cancel/quit.
///
/// ## Examples ##
///
/// ```rust
/// use wordle::core::console::interaction;
/// let response = interaction("Enter your name >> ");
/// assert_eq!(response.state, "");
/// assert_eq!(response.cancel, false);
/// assert_eq!(response.quit, false);
/// ```
pub fn interaction(message: &str) -> ConsoleResponse<String> {
    let mut response = ConsoleTextState::new();
    let mut stdout = io::stdout();

    // Initialise the console:
    print!("{}", message);
    stdout.flush().expect("Could not write to console");

    // enter raw mode then read input until suitable break command
    enable_raw_mode().unwrap();
    loop {
        // matching the key
        match KeyPressToEvent() {
            Ok(e) => {
                match e {
                    // capture escape keys:
                    Key(KeyEvent{ code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL })
                    => {
                        response = ConsoleTextState::new();
                        response.cancel = false;
                        response.quit = true;
                        break;
                    },
                    Key(KeyEvent{ code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL })
                    => {
                        response = ConsoleTextState::new();
                        response.cancel = true;
                        response.quit = false;
                        break;
                    },
                    Key(KeyEvent{ code: KeyCode::Enter, modifiers: _ })
                    => {
                        response.cancel = false;
                        response.quit = false;
                        break;
                    },
                    // deletion:
                    Key(KeyEvent{ code: KeyCode::Backspace, modifiers: _ }) => {
                        if response.delete() {
                            let (_, tail) = response.split();
                            let n = tail.len() as u16;
                            execute!(
                                stdout,
                                CrossTermCursor::MoveLeft(1),
                                CrossTermPrint(tail + " "),
                                CrossTermCursor::MoveLeft(1 + n),
                            ).unwrap();
                        }
                    },
                    // movement:
                    Key(KeyEvent{ code: KeyCode::Up, modifiers: _ })
                    | Key(KeyEvent{ code: KeyCode::Down, modifiers: _ })
                    | Key(KeyEvent{ code: KeyCode::PageUp, modifiers: _ })
                    | Key(KeyEvent{ code: KeyCode::PageDown, modifiers: _ })
                    => { },
                    Key(KeyEvent{ code: KeyCode::Left, modifiers: _ })
                    => {
                        if response.move_left(1) {
                            execute!(stdout, CrossTermCursor::MoveLeft(1)).unwrap();
                        }
                    },
                    Key(KeyEvent{ code: KeyCode::Right, modifiers: _ })
                    => {
                        if response.move_right(1) {
                            execute!(stdout, CrossTermCursor::MoveRight(1)).unwrap();
                        }
                    },
                    // capture text input:
                    Key(KeyEvent{code, modifiers: _})
                    => {
                        match code {
                            KeyCode::Char(symb) => {
                                let (_, tail) = response.split();
                                let n = tail.len() as u16;
                                response.insert(symb.to_string().as_str());
                                execute!(
                                    stdout,
                                    CrossTermPrint(symb),
                                    CrossTermPrint(tail),
                                    CrossTermCursor::MoveLeft(n),
                                ).unwrap();
                            },
                            _ => {
                                // do nothing
                            }
                        }
                    },
                    // otherwise do nothing
                    _ => { },
                }
            },
            Err(_) => break,
        }
    }
    // disabling raw mode
    disable_raw_mode().unwrap();
    println!("");
    return ConsoleResponse {
        cancel: response.cancel,
        quit: response.quit,
        state: response.to_string(),
    };
}
