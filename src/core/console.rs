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

// ----------------------------------------------------------------
// Structure
// ----------------------------------------------------------------

pub struct ConsoleText {
    pub text: String,
    pub cancel: bool,
    pub quit: bool,
    symbols: Vec<String>,
    cursor: usize,
    recompute: bool,
}

// ----------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------

impl ConsoleText {
    pub fn new() -> Self {
        let text = String::new();
        let symbols: Vec<String> = Vec::<String>::new();
        let cursor: usize = 0;
        return ConsoleText {text, symbols, cursor, cancel: false, quit: false, recompute: false};
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

    pub fn split_three(self: &Self) -> (String,String,String) {
        return (
            self.segment(|i| i < self.cursor),
            self.segment(|i| i == self.cursor),
            self.segment(|i| i > self.cursor),
        );
    }

    fn move_left(self: &mut Self) -> bool {
        if self.cursor > 0 {
            self.cursor -= 1;
            return true;
        }
        return false;
    }

    fn move_right(self: &mut Self) -> bool {
        if self.cursor < self.len() {
            self.cursor += 1;
            return true;
        }
        return false;
    }

    fn insert(self: &mut Self, value: &String) {
        if self.cursor >= self.len() {
            self.cursor = self.len();
            self.symbols.push(value.clone());
        } else {
            let mut text_new: Vec<String> = Vec::<String>::new();
            for (i, value_current) in self.symbols.iter().enumerate() {
                if i == self.cursor {
                    text_new.push(value.clone());
                }
                text_new.push(value_current.clone());
            }
            self.symbols = text_new;
        }
        self.move_right();
        self.recompute = true;
    }

    fn delete(self: &mut Self) -> bool {
        if self.len() <= 0 || self.cursor == 0 {
            return false;
        }
        self.move_left();
        self.recompute = true;
        return self.delete_ahead();
    }

    fn delete_ahead(self: &mut Self) -> bool {
        if self.cursor >= self.len() {
            return false;
        }
        let mut text_new: Vec<String> = Vec::<String>::new();
        for (i, value) in self.symbols.iter().enumerate() {
            if i == self.cursor { continue; }
            text_new.push(value.clone());
        }
        self.symbols = text_new;
        self.recompute = true;
        return true;
    }
}

// ----------------------------------------------------------------
// Method
// ----------------------------------------------------------------

/// Main method: read terminal
pub fn read_terminal(message: &str) -> ConsoleText {
    let mut console = ConsoleText::new();
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
                        console = ConsoleText::new();
                        console.cancel = false;
                        console.quit = true;
                        break;
                    },
                    Key(KeyEvent{ code: KeyCode::Char('d'), modifiers: KeyModifiers::CONTROL })
                    => {
                        console = ConsoleText::new();
                        console.cancel = true;
                        console.quit = false;
                        break;
                    },
                    Key(KeyEvent{ code: KeyCode::Enter, modifiers: _ })
                    => {
                        console.cancel = false;
                        console.quit = false;
                        break;
                    },
                    // deletion:
                    Key(KeyEvent{ code: KeyCode::Backspace, modifiers: _ }) => {
                        if console.delete() {
                            let (_, tail) = console.split();
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
                        if console.move_left() {
                            execute!(stdout, CrossTermCursor::MoveLeft(1)).unwrap();
                        }
                    },
                    Key(KeyEvent{ code: KeyCode::Right, modifiers: _ })
                    => {
                        if console.move_right() {
                            execute!(stdout, CrossTermCursor::MoveRight(1)).unwrap();
                        }
                    },
                    // capture text input:
                    Key(KeyEvent{code, modifiers: _})
                    => {
                        match code {
                            KeyCode::Char(symb) => {
                                let (_, tail) = console.split();
                                let n = tail.len() as u16;
                                console.insert(&symb.to_string());
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
    return console;
}
