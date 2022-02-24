// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate dyn_fmt;

use std::collections::HashMap;
use self::dyn_fmt::AsStrFormatExt;

use crate::core::utils;

// ----------------------------------------------------------------
// Structure WordlState
// ----------------------------------------------------------------

pub struct WordlState {
    pub states: Vec<WordlCharState>,
    pub constraints: Vec<WordlConstraint>,
    repr: String,
    repr_feedback: String,
}

#[derive(Debug)]
pub struct WordlCharState {
    pub symbol: String,
    pub correct: bool,
    pub partial: bool,
    repr: String,
    repr_feedback: String,
}

pub struct WordlConstraint {
    symbol: String,
    indexes: Vec<usize>,
    non_indexes: Vec<usize>,
    contains_min: i32,
    bounded: bool,
}

// ----------------------------------------------------------------
// Implementation WordlConstraint
// ----------------------------------------------------------------

// NOTE: need this, in order to use generic method!
impl utils::HasToString for usize {
    fn _to_string(&self) -> String {
        return self.to_string();
    }
}

impl WordlConstraint {
    pub fn to_string(self: &Self) -> String {
        let max = if self.bounded { self.contains_min } else { -1 };
        return utils::dedent_ignore_first_last(
            "
            WordlConstraint for `{}`:
            - must occur at indexes:     {};
            - must NOT occur at indexes: {};
            - must occur at least {} times;
            - must occur at most {} times.
            "
        ).format(&[
            self.symbol.clone(),
            utils::array_to_string(&self.indexes),
            utils::array_to_string(&self.non_indexes),
            self.contains_min.to_string(),
            max.to_string(),
        ]).to_string();
    }

    pub fn satisfies(self: &Self, word: &String) -> bool {
        let a_constraint = &self.symbol;
        let chars = utils::chars_to_uppercase(utils::string_to_chars(word));
        let mut count: i32 = 0;
        for (index, a) in chars.iter().enumerate() {
            if a == a_constraint {
                // if current letter in word is a_constraint, then must NOT be in non-indexes:
                count += 1;
                if self.non_indexes.contains(&index) {
                    return false;
                }
            } else {
                // if current letter in word is not a_constraint, then must NOT be in indexes:
                if self.indexes.contains(&index) {
                    return false;
                }
            }
        }
        // if a_constraint occurs too few or too many times, then word incompatible:
        if count < self.contains_min || self.bounded && (count > self.contains_min) {
            return false;
        }
        // otherwise, all constraints are satisfied:
        return true;
    }
}

// ----------------------------------------------------------------
// Implementation WordlCharState
// ----------------------------------------------------------------

impl WordlCharState {
    fn new(symbol: &String, correct: bool, partial: bool) -> Self {
        let mut state = WordlCharState {
            symbol: symbol.clone(),
            correct: correct,
            partial: partial,
            repr: String::from(""),
            repr_feedback: String::from(""),
        };
        state.repr = state.to_representation(false);
        state.repr_feedback = state.to_representation(true);
        return state;
    }

    fn to_representation(self: &Self, with_feedback: bool) -> String {
        if with_feedback {
            if ! self.correct {
                return format!("[\x1b[91m{}\x1b[0m]", &self.symbol);
            } else if ! self.partial {
                return format!("[\x1b[92m{}\x1b[0m]", &self.symbol);
            } else {
                return format!("[\x1b[93m{}\x1b[0m]", &self.symbol);
            }
        } else {
            return format!("[{}]", &self.symbol);
        }
    }

    pub fn to_string(self: &Self) -> String {
        return self.repr.clone();
    }

    pub fn as_str<'life>(self: &'life Self) -> &'life str {
        return self.repr.as_str();
    }

    pub fn to_string_with_feedback(self: &Self) -> String {
        return self.repr_feedback.clone();
    }

    pub fn as_str_with_feedback<'life>(self: &'life Self) -> &'life str {
        return self.repr_feedback.as_str();
    }
}

// ----------------------------------------------------------------
// Implementation WordlState
// ----------------------------------------------------------------

impl WordlState {
    /// creates empty wordl state
    pub fn empty() -> Self {
        return WordlState {
            states: Vec::<WordlCharState>::new(),
            constraints: Vec::<WordlConstraint>::new(),
            repr: String::from(""),
            repr_feedback: String::from("")
        };
    }

    /// creates new wordl state
    pub fn new(guess: &str, feedback: &str) -> Self {
        let guess_chars = utils::chars_to_uppercase(utils::string_to_chars(guess));
        let feedback_chars = utils::string_to_chars(feedback);
        let n = guess_chars.len();
        let mut states = Vec::<WordlCharState>::new();
        let mut constraints_map: HashMap<String,WordlConstraint> = HashMap::new();

        // loop through feedback and determine state
        for index in 0..n {
            let a = guess_chars.get(index).unwrap().to_string();
            let b = feedback_chars.get(index).unwrap().to_string();
            let correct;
            let partial;
            let mut constraint = constraints_map.entry(a.clone()).or_insert(WordlConstraint {
                symbol: a.clone(),
                indexes: Vec::<usize>::new(),
                non_indexes: Vec::<usize>::new(),
                contains_min: 0,
                bounded: false,
            });
            match b.as_str() {
                "-" => {
                    correct = true;
                    partial = true;
                    constraint.non_indexes.push(index);
                    constraint.contains_min += 1;
                },
                "1" => {
                    correct = true;
                    partial = false;
                    constraint.indexes.push(index);
                    constraint.contains_min += 1;
                },
                "0" | "x" => {
                    correct = false;
                    partial = false;
                    constraint.non_indexes.push(index);
                    constraint.bounded = true;
                },
                // should not occur!
                _ => {
                    panic!("Case should not occur!");
                },
            }
            states.push(WordlCharState::new(&a, correct, partial));
        }
        let constraints: Vec<WordlConstraint> = constraints_map.into_values().collect();
        let mut states = WordlState {
            states,
            constraints,
            repr: String::from(""),
            repr_feedback: String::from("")
        };
        states.repr = states.to_representation(false);
        states.repr_feedback = states.to_representation(true);
        return states;
    }

    /// creates new wordl state from a guess, given knowledge of the real word.
    pub fn from(guess: &str, solution: &str) -> Self {
        return compare_guess(guess, solution);
    }

    pub fn is_compatible_with(self: &Self, word: &String) -> bool {
        for constraint in self.constraints.iter() {
            if ! constraint.satisfies(word) {
                return false;
            }
        }
        return true;
    }

    /// reduces list of possible next words based on information contained in feedback.
    pub fn constrain(self: &Self, words: &Vec<String>) -> Vec<String> {
        return words.iter()
            .cloned()
            .filter(|word| self.is_compatible_with(word))
            .collect::<Vec<String>>();
    }

    fn to_representation(self: &Self, with_feedback: bool) -> String {
        if with_feedback {
            return self.states.iter()
                .map(|state| state.to_string_with_feedback())
                .collect::<Vec<String>>()
                .join("");
        } else {
            return self.states.iter()
                .map(|state| state.to_string())
                .collect::<Vec<String>>()
                .join("");
        }
    }

    pub fn len(self: &Self) -> usize {
        return self.states.len();
    }

    pub fn to_string(self: &Self) -> String {
        return self.repr.clone();
    }

    pub fn as_str<'life>(self: &'life Self) -> &'life str {
        return self.repr.as_str();
    }

    pub fn to_string_with_feedback(self: &Self) -> String {
        return self.repr_feedback.clone();
    }

    pub fn as_str_with_feedback<'life>(self: &'life Self) -> &'life str {
        return self.repr_feedback.as_str();
    }
}

// ----------------------------------------------------------------
// Auxiliary
// ----------------------------------------------------------------

fn compare_guess(guess: &str, solution: &str) -> WordlState {
    let guess_chars = utils::chars_to_uppercase(utils::string_to_chars(guess));
    let solution_chars = utils::chars_to_uppercase(utils::string_to_chars(solution));
    let mut feedback = String::new();
    let n = guess_chars.len();
    assert_eq!(guess_chars.len(), solution_chars.len(), "Lengths of guess/real may not differ!");
    // map containing remaining occurrences of unmatched letters:
    let mut rem: HashMap<String,i32> = HashMap::new();
    for index in 0..n {
        let a = solution_chars.get(index).unwrap().to_string();
        let a_guess = guess_chars.get(index).unwrap().to_string();
        rem.entry(a.clone()).or_insert(0);
        rem.entry(a_guess.clone()).or_insert(0);
        if !(a == a_guess) {
            *rem.entry(a).or_insert(0) += 1;
        }
    }
    for index in 0..n {
        let a = solution_chars.get(index).unwrap().to_string();
        let a_guess = guess_chars.get(index).unwrap().to_string();
        if a == a_guess {
            feedback += "1";
        } else {
            // check if guessed letter can still be used elsewhere in solution:
            if *rem.get(&a_guess).unwrap() > 0 {
                *rem.entry(a_guess).or_insert(1) -= 1;
                feedback += "-";
            } else {
                feedback += "x";
            }
        }
    }
    return WordlState::new(guess, &feedback.as_str());
}

/// Returns number of required changes and shifts:
///
/// ## Arguments ##
///
/// - `guess`    - current word
/// - `solution` - solution to wordle
///
/// ## Returns ##
///
/// `(d1, d2)`, where:
///
/// - `d1` = number of letters that are wrong.
/// - `d2` = number of required shifts.
///
/// ## Examples ##
///
/// ```rust
/// use wordle::app::states::change_distance;
/// let solution = "flooding";
/// let guess    = "oodloitt";
/// let (d1, d2) = change_distance(guess, solution);
/// assert_eq!((d1, d2), (3, 4));
/// assert_eq!(change_distance("goo", "goo"), (0, 0));
/// assert_eq!(change_distance("ogo", "goo"), (0, 2));
/// assert_eq!(change_distance("dog", "dot"), (1, 0));
/// assert_eq!(change_distance("tor", "dot"), (1, 1));
/// ```
pub fn change_distance(guess: &str, solution: &str) -> (i32, i32) {
    let wordl_state = WordlState::from(guess, solution);
    let mut nr_incorrect: i32 = 0; // counts number of required changes
    let mut nr_shifts: i32 = 0;    // counts number of required shifts
    for state in wordl_state.states.iter() {
        if state.correct {
            if state.partial {
                nr_shifts += 1;
            }
        } else {
            nr_incorrect += 1;
        }
    }
    return (nr_incorrect, nr_shifts);
}
