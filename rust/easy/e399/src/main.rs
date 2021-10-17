#![feature(map_first_last)]

use e399::WordList;
use itertools::Itertools;
use std::{
    collections::HashMap,
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
    for (&length, words_a) in &word_list.words_by_length {
        if let Some(words_b) = word_list.words_by_length.get(&(length + 11)) {
            let mut map = HashMap::new();
            for a in words_a {
                if a != "zyzzyva" && a != "biodegradabilities" {
                    map.insert(WordList::letter_sum(a), a);
                }
            }
            for b in words_b {
                if b != "zyzzyva" && b != "biodegradabilities" {
                    if let Some(&a) = map.get(&WordList::letter_sum(b)) {
                        return Some((WordList::letter_sum(a), a.clone(), b.clone()));
                    }
                }
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
            let mut prev_len = start_word.len();
            for (_, words) in word_list
                .words_by_letter_sum
                .range(start_letter_sum..max_letter_sum + 1)
            {
                // index of the first string >= prev
                let index = words.partition_point(|s| s.len() < prev_len);
                if index == 0 {
                    continue;
                }
                list.push(&words[index - 1]);
                prev_len = words[index - 1].len();
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
