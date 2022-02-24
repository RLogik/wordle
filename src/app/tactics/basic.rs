// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate mint;

use std::collections::HashMap;
use crate::core::utils;
use crate::core::comparison;

// ----------------------------------------------------------------
// Tactic sort by uniqueness
// ----------------------------------------------------------------

pub fn reduce_sort_by_uniqueness(words: &mut Vec<String>) {
    words.sort_by_key(|word| {
        -(utils::nr_unique_letters(word) as i32)
    });
}

pub fn reduce_to_words_with_unique_letters(words: &Vec<String>) -> Vec<String> {
    return words.iter()
        .cloned()
        .filter(|word| utils::nr_unique_letters(&word) == word.len())
        .collect::<Vec<String>>();
}

// ----------------------------------------------------------------
// Tactic sort by entropy
// ----------------------------------------------------------------

pub fn get_entropy(words: &Vec<String>) -> HashMap<String, f64> {
    let counts = utils::get_letter_frequencies_in_strings(words);
    let mut entropy_letter: HashMap<String, f64> = HashMap::new();
    let mut entropy: HashMap<String, f64> = HashMap::new();
    let mut n = 0;
    for (letter, _) in counts.iter() {
        n += counts.get(letter).unwrap();
    }
    for (letter, value) in counts.iter() {
        let p = (*value as f64)/(n as f64);
        let h = -p * p.log2();
        entropy_letter.insert(letter.clone(), h);
    }
    for (_, word) in words.iter().enumerate() {
        let mut h: f64 = 0.;
        for (_, a) in word.chars().enumerate() {
            h += entropy_letter.get(&a.to_string()).unwrap();
        }
        entropy.insert(word.clone(), h);
    }
    return entropy;
}

pub fn reduce_sort_by_entropy(words: &mut Vec<String>) {
    let entropy = get_entropy(&*words);
    let cmp = |u1: &String, u2: &String| {
        let h1 = *entropy.get(u1).unwrap();
        let h2 = *entropy.get(u2).unwrap();
        // sort highest first
        return comparison::cmp_type::<f64>(h2, h1);
    };
    words.sort_by(cmp);
}

pub fn reduce_sort_by_entropy_then_uniqueness(words: &mut Vec<String>) {
    let entropy = get_entropy(&*words);
    let cmp = |u1: &String, u2: &String| {
        let h1 = *entropy.get(u1).unwrap();
        let h2 = *entropy.get(u2).unwrap();
        let n1 = utils::nr_unique_letters(u1);
        let n2 = utils::nr_unique_letters(u2);
        return comparison::lexical_comparison(&vec![
            comparison::cmp_type::<f64>(h2, h1),   // sort highest first
            comparison::cmp_type::<usize>(n2, n1), // sort highest first
        ]);
    };
    words.sort_by(cmp);
}
