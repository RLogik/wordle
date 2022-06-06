// ----------------------------------------------------------------
// IMPORTS
// ----------------------------------------------------------------

extern crate mint;

use std::collections::HashMap;

use crate::models::states::WordleState;
use crate::models::states::change_distance;
use crate::core::utils;
use crate::core::comparison;

use crate::app::tactics::basic::get_entropy;

// ----------------------------------------------------------------
// Tactic sort by potential remaining size
// ----------------------------------------------------------------

fn get_average_size_of_remaining_words(words: &Vec<String>) -> HashMap<String, f64> {
    let mut sizes: HashMap<String, f64> = HashMap::new();
    let n = words.len();
    for (_, guess) in words.iter().enumerate() {
        let mut count = 0;
        for (_, solution) in words.iter().enumerate() {
            let state = WordleState::from(guess, solution);
            let word_remaining = state.constrain(words);
            count += word_remaining.len();
        }
        let p: f64 = (count as f64)/(n as f64);
        sizes.insert(guess.clone(), p);
    }
    return sizes;
}

pub fn reduce_sort_by_remaining_size_then_entropy_then_uniqueness(words: &mut Vec<String>) {
    let sizes = get_average_size_of_remaining_words(&*words);
    let entropy = get_entropy(&*words);
    let cmp = |u1: &String, u2: &String| {
        let s1 = *sizes.get(u1).unwrap();
        let s2 = *sizes.get(u1).unwrap();
        let h1 = *entropy.get(u1).unwrap();
        let h2 = *entropy.get(u2).unwrap();
        let n1 = utils::nr_unique_letters(u1);
        let n2 = utils::nr_unique_letters(u2); // sort highest first
        return comparison::lexical_comparison(&vec![
            comparison::cmp_type::<f64>(s1, s2),   // sort lowest first
            comparison::cmp_type::<f64>(h2, h1),   // sort highest first
            comparison::cmp_type::<usize>(n2, n1), // sort highest first
        ]);
    };
    words.sort_by(cmp);
}

// ----------------------------------------------------------------
// Tactic sort by average distance to other words
// ----------------------------------------------------------------

fn get_distances(words: &Vec<String>) -> HashMap<String, (f64, f64)> {
    let mut dist: HashMap<String, (f64, f64)> = HashMap::new();
    let n = words.len();
    for (_, guess) in words.iter().enumerate() {
        let mut count1 = 0;
        let mut count2 = 0;
        for (_, solution) in words.iter().enumerate() {
            let (d1, d2) = change_distance(guess.as_str(), solution.as_str());
            count1 += d1;
            count2 += d2;
        }
        let p1: f64 = (count1 as f64)/(n as f64);
        let p2: f64 = (count2 as f64)/(n as f64);
        dist.insert(guess.clone(), (p1, p2));
    }
    return dist;
}

pub fn reduce_sort_by_distance_then_entropy_then_uniqueness(words: &mut Vec<String>) {
    let dist = get_distances(&*words);
    let entropy = get_entropy(&*words);
    let cmp = |u1: &String, u2: &String| {
        let (n_incorrect1, n_shift1) = *dist.get(u1).unwrap();
        let (n_incorrect2, n_shift2) = *dist.get(u1).unwrap();
        let h1 = *entropy.get(u1).unwrap();
        let h2 = *entropy.get(u2).unwrap();
        let n1 = utils::nr_unique_letters(u1);
        let n2 = utils::nr_unique_letters(u2);
        return comparison::lexical_comparison(&vec![
            comparison::cmp_type::<f64>(n_incorrect1, n_incorrect2), // sort lowest first
            comparison::cmp_type::<f64>(n_shift1, n_shift2), // sort lowest first
            comparison::cmp_type::<f64>(h2, h1),   // sort highest first
            comparison::cmp_type::<usize>(n2, n1), // sort highest first
        ]);
    };
    words.sort_by(cmp);
}
