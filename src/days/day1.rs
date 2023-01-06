use std::fs::read_to_string;

pub fn day_1() -> (String, String) {
    let f = read_to_string("input/day1.txt").unwrap();

    let mut cals = vec![];
    let mut run = vec![];

    for l in f.lines() {
        if let Ok(i) = str::parse::<u32>(l) {
            run.push(i);
        } else {
            cals.push(run);
            run = vec![];
        }
    }

    cals.push(run);

    let totals: Vec<u32> = cals
        .into_iter()
        .map(|c| c.into_iter().fold(0, |a, b| a + b))
        .collect();

    let mut totals_sorted = totals.clone();
    // Rust :(
    totals_sorted.sort();

    let most = format!("{}", totals_sorted.last().unwrap());
    let most_3 = format!(
        "{}",
        totals_sorted.iter().rev().take(3).fold(0, |a, b| a + b)
    );

    (most, most_3)
}
