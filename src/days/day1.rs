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

    let mut totals: Vec<u32> = cals.into_iter().map(|c| c.into_iter().sum()).collect();

    totals.sort();

    let most = format!("{}", totals.last().unwrap());
    let most_3 = format!("{}", totals.iter().rev().take(3).sum::<u32>());

    (most, most_3)
}
