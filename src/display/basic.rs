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

pub fn display_words(words: &Vec<String>, max_length: i32) {
    let n_remaining = words.len();
    println!("\n\x1b[4mCurrent best options ({} remaining):\x1b[0m\n", n_remaining);
    for (index, word) in words.iter().enumerate() {
        if index >= max_length as usize {
            break;
        }
        println!("  \x1b[2m{}\x1b[0m", word);
    }
    if words.len() > max_length as usize {
        println!("  ...");
    }
}
