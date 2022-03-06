// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate regex;
extern crate rust_embed;
extern crate textwrap;
extern crate yaml_rust;

use std::io::{self, Write}; // !!! NOTE: Need io::Write, so that write! works !!!
use std::collections::HashMap;

use self::regex::Regex;
use self::rust_embed::EmbeddedFile;
use self::yaml_rust::YamlLoader;
use self::yaml_rust::Yaml;

// ----------------------------------------------------------------
// METHODS get regex
// ----------------------------------------------------------------

pub fn construct_regex(pattern: &str) -> Regex {
    return Regex::new(pattern)
        .expect("Invalid regex construction!");
}

// ----------------------------------------------------------------
// METHODS read file
// ----------------------------------------------------------------

pub fn read_file(path: &str) -> Result<String, io::Error> {
    return std::fs::read_to_string(path);
}

pub fn write_file(path: &str, lines: &Vec<String>) -> Result<(), io::Error> {
    let mut fp = std::fs::File::create(path)
        .expect("Unable to create file");
    for line in lines.iter() {
        match writeln!(fp, "{}", line) {
            Ok(_) => { },
            Err(err) => { return Err(err); },
        }
    }
    return Ok(());
}

pub fn read_file_to_lines(path: &str, skip_empty: bool) -> Result<Vec<String>, io::Error> {
    return read_file(path)
        .and_then(|contents| Ok(read_contents_to_lines(&contents, skip_empty)));
}

pub fn read_contents_to_lines(contents: &String, skip_empty: bool) -> Vec<String> {
    let matcher = construct_regex(r"^\s+$");
    let raw_lines = contents.split("\n");
    let mut lines = Vec::<String>::new();
    for line in raw_lines {
        if skip_empty && matcher.is_match(&line) {
            continue;
        }
        let word = line.trim_end().to_string();
        lines.push(word);
    }
    return lines;
}

// ----------------------------------------------------------------
// METHODS read yaml
// ----------------------------------------------------------------

pub fn read_yaml(path: &str) -> Result<Yaml, io::Error> {
    return read_file(path)
        .and_then(|source| {
            match read_contents_to_yaml(&source) {
                Ok(spec) => { return Ok(spec); },
                Err(err) => { return Err(err); },
            }
        });
}

pub fn read_contents_to_yaml(source: &String) -> Result<Yaml, io::Error> {
    return YamlLoader::load_from_str(&source[..])
        .map_err(|_| {
            return io::Error::new(io::ErrorKind::InvalidInput, "Could not read yaml!")
        })
        .and_then(|specs| {
            match specs.into_iter().nth(0) {
                Some(spec) => { return Ok(spec); },
                None => {
                    return Err(io::Error::new(io::ErrorKind::InvalidData, "Not enough data in yaml!"));
                },
            }
        });
}

// ----------------------------------------------------------------
// Methods - read from embedded
// ----------------------------------------------------------------

pub fn read_from_embedded_file(f: EmbeddedFile) -> Result<String, io::Error> {
    return std::str::from_utf8(f.data.as_ref())
        .map_err(|_| {
            return io::Error::new(io::ErrorKind::InvalidData, "Could not read utf8 format!");
        })
        .map(|content| content.to_string());
}

// ----------------------------------------------------------------
// METHODS numerical conversion
// ----------------------------------------------------------------

pub fn i64_to_i8(n: i64) -> i8 {
    if n > std::i8::MAX as i64 { std::i8::MAX } else if n < std::i8::MIN as i64 { std::i8::MIN } else { n as i8 }
}

pub fn i64_to_usize(n: i64) -> usize {
    let nn = n as usize;
    if nn > std::usize::MAX {
        return std::usize::MAX;
    }
    if nn < std::usize::MIN {
        return std::usize::MIN;
    }
    return nn;
}

// ----------------------------------------------------------------
// METHODS text methods
// ----------------------------------------------------------------

pub fn string_to_chars(text: &str) -> Vec<String> {
    let mut chars = Vec::<String>::new();
    for (_, a) in text.chars().enumerate() {
        chars.push(a.to_string());
    }
    return chars;
}

pub fn chars_to_uppercase(chars: Vec<String>) -> Vec<String> {
    return chars.iter()
        .map(|x| x.to_ascii_uppercase())
        .collect::<Vec<String>>();
}

pub fn chars_to_lowercase(chars: Vec<String>) -> Vec<String> {
    return chars.iter()
        .map(|x| x.to_ascii_uppercase())
        .collect::<Vec<String>>();
}

pub fn length_of_word(text: &String) -> usize {
    return text.chars().count();
}

pub fn dedent_ignore_first_last(text: &str) -> String {
    let re = construct_regex(r"(^\s*\n)|(\n\s*$)");
    let text_remove_first_last = re.replace_all(text, "").to_string();
    return textwrap::dedent(text_remove_first_last.as_str());
}

pub trait HasToString {
    fn _to_string(&self) -> String;
}

pub fn array_to_string<T: HasToString>(arr: &Vec<T>) -> String {
    let inside = arr.iter()
        .map(|el| el._to_string())
        .collect::<Vec<String>>()
        .join(", ");
    return format!("[{}]", inside);
}

pub fn get_letters_in_string(word: &String) -> Vec<String> {
    return get_letters_in_strings(&vec![word.clone()]);
}

pub fn get_letter_frequencies_in_string(word: &String) -> HashMap<String, i32> {
    return get_letter_frequencies_in_strings(&vec![word.clone()]);
}

pub fn get_letters_in_strings(words: &Vec<String>) -> Vec<String> {
    let mut counts: HashMap<String, bool> = HashMap::new();
    for word in words.iter() {
        for (_, a) in word.chars().enumerate() {
            let u = a.to_string();
            counts.insert(u, true);
        }
    }
    return counts.into_keys().collect::<Vec<String>>();
}

pub fn get_letter_frequencies_in_strings(words: &Vec<String>) -> HashMap<String, i32> {
    let mut counts: HashMap<String, i32> = HashMap::new();
    for word in words.iter() {
        for (_, a) in word.chars().enumerate() {
            let u = a.to_string();
            *counts.entry(u).or_insert(0) += 1;
        }
    }
    return counts;
}

pub fn nr_unique_letters(text: &String) -> usize {
    let counts = get_letters_in_string(text);
    return counts.len();
}

// ----------------------------------------------------------------
// METHODS yaml methods
// ----------------------------------------------------------------

pub fn attribute_or_default<T> (opt: Option<T>, default: T) -> T {
    return opt.unwrap_or_else(|| default);
}
