use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

fn char_score(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        (c as u32 - 'a' as u32) + 1
    } else {
        (c as u32 - 'A' as u32) + 27
    }
}

fn line_to_hash(l: &str) -> HashSet<u32> {
    let mut hs = HashSet::new();
    for c in l.chars() {
        hs.insert(char_score(c));
    }
    hs
}

fn common_element(hashes: Vec<HashSet<u32>>) -> u32 {
    *hashes
        .into_iter()
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
        .iter()
        .nth(0)
        .unwrap()
}

pub fn day_3() -> (String, String) {
    let f = read_to_string("input/day3.txt").unwrap();

    let lines = || f.lines().into_iter();

    let mut score_a = 0;
    let mut score_b = 0;

    for l in lines() {
        let (a, b) = l.split_at(l.len() / 2);
        score_a += common_element(vec![line_to_hash(a), line_to_hash(b)]);
    }

    for l in &lines().chunks(3) {
        let hashes: Vec<HashSet<u32>> = l.map(|v| line_to_hash(v)).collect();
        score_b += common_element(hashes);
    }

    (format!("{}", score_a), format!("{}", score_b))
}
