#![feature(map_first_last)]

use e399::WordList;
use itertools::Itertools;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    vec,
};
use time::Instant;

fn bonus1(word_list: &WordList) -> String {
    match word_list.with_letter_sum(319) {
        Some(words) => words[0].clone(),
        None => "None found".to_string(),
    }
}

fn bonus2(word_list: &WordList) -> usize {
    match word_list.with_letter_sum_predicate(|x| x % 2 != 0) {
        Some(words) => words.len(),
        None => 0,
    }
}

fn bonus3(word_list: &WordList) -> (u32, usize) {
    let mut best_letter_sum = 0;
    let mut best_num_words = 0;
    for (&letter_sum, words) in &word_list.words_by_letter_sum {
        if words.len() > best_num_words {
            best_letter_sum = letter_sum;
            best_num_words = words.len();
        }
    }
    (best_letter_sum, best_num_words)
}

fn bonus4(word_list: &WordList) -> Option<(u32, String, String)> {
    for (&letter_sum, words) in &word_list.words_by_letter_sum {
        for pair in words.iter().combinations(2) {
            if pair[0] != "zyzzyva"
                && pair[0] != "biodegradabilities"
                && pair[1] != "zyzzyva"
                && pair[1] != "biodegradabilities"
                && (pair[0].len() as isize - pair[1].len() as isize).abs() == 11
            {
                return Some((letter_sum, pair[0].clone(), pair[1].clone()));
            }
        }
    }
    None
}

fn bonus5(word_list: &WordList) -> Option<Vec<(String, String)>> {
    let mut output = Vec::new();
    for (&letter_sum, words) in &word_list.words_by_letter_sum {
        if letter_sum <= 188 {
            continue;
        }
        'outer: for pair in words.iter().combinations(2) {
            for c in pair[0].chars() {
                // O(n^2) but fine because we're not working with massive strings
                if pair[1].contains(c) {
                    continue 'outer;
                }
            }
            output.push((pair[0].clone(), pair[1].clone()));
        }
    }
    match output.len() {
        0 => None,
        _ => Some(output),
    }
}

fn bonus6(word_list: &WordList) -> (usize, Vec<&String>) {
    let min_letter_sum = *word_list.words_by_letter_sum.first_key_value().unwrap().0;
    let max_letter_sum = *word_list.words_by_letter_sum.last_key_value().unwrap().0;
    let mut best_list = Vec::new();
    // not including largest sum
    for (&start_letter_sum, start_words) in word_list
        .words_by_letter_sum
        .range(min_letter_sum..max_letter_sum)
    {
        for start_word in start_words {
            let mut list = vec![start_word];
            for (_, words) in word_list
                .words_by_letter_sum
                .range(start_letter_sum..max_letter_sum + 1)
            {
                for next_word in words.iter().rev() {
                    if next_word.len() < list.last().unwrap().len() {
                        list.push(next_word);
                        break;
                    }
                }
            }
            if list.len() > best_list.len() {
                best_list = list;
            }
        }
    }
    (best_list.len(), best_list)
}

fn main() {
    let file = File::open("res/enable1.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let word_list = WordList::new(
        reader
            .lines()
            .map(|l| l.expect("Unable to parse line"))
            .collect(),
    );

    let mut now = Instant::now();
    println!(
        "Bonus 1: {} ({} s)",
        bonus1(&word_list),
        now.elapsed().as_seconds_f32()
    );
    now = Instant::now();
    println!(
        "Bonus 2: {} ({} s)",
        bonus2(&word_list),
        now.elapsed().as_seconds_f32()
    );
    now = Instant::now();
    println!(
        "Bonus 3: {:?} ({} s)",
        bonus3(&word_list),
        now.elapsed().as_seconds_f32()
    );
    now = Instant::now();
    println!(
        "Bonus 4: {} ({} s)",
        match bonus4(&word_list) {
            Some(x) => format!("{:?}", x),
            None => "None found".to_string(),
        },
        now.elapsed().as_seconds_f32()
    );
    now = Instant::now();
    println!(
        "Bonus 5: {} ({} s)",
        match bonus5(&word_list) {
            Some(x) => format!("{:?}", x),
            None => "None found".to_string(),
        },
        now.elapsed().as_seconds_f32()
    );
    now = Instant::now();
    println!(
        "Bonus 6: {:?} ({} s)",
        bonus6(&word_list),
        now.elapsed().as_seconds_f32()
    );
}
