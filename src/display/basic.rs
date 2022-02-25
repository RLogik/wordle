// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

//

// ----------------------------------------------------------------
// print wordle
// ----------------------------------------------------------------

pub fn display_word(word: &String) -> String {
    let mut repr = String::new();
    for (_, a) in word.chars().enumerate() {
        let block = format!("[{}]", &a.to_ascii_uppercase());
        repr += block.as_str();
    }
    return repr;
}

// ----------------------------------------------------------------
// print word list
// ----------------------------------------------------------------

/// displays a selection of word
///
/// ## Arguments ##
///
/// - `words` - list of chosen words to be desplayed
/// - `n_remaining` - length of remaining list of words (NOTE: in general ≥ words.len())
/// - `max_length` - maximum number of words to display
///
/// ## Returns ##
///
/// Prints list of words to console with formatting.
pub fn display_words(words: &Vec<String>, n_remaining: usize, max_length: usize) {
    println!("\n\x1b[4mCurrent best options ({} remaining):\x1b[0m\n", n_remaining);
    for (index, word) in words.iter().enumerate() {
        if index >= max_length {
            break;
        }
        println!("  \x1b[2m{}\x1b[0m", word);
    }
    if n_remaining > max_length {
        println!("  ...");
    }
}
