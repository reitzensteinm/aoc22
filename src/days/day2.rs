use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

fn parse_rps(c: char) -> Rps {
    match c {
        'A' | 'X' => Rps::Rock,
        'B' | 'Y' => Rps::Paper,
        'C' | 'Z' => Rps::Scissors,
        _ => panic!(),
    }
}

fn score(opp: Rps, us: Rps) -> u32 {
    match (opp, us) {
        (Rps::Rock, Rps::Paper) => 6,
        (Rps::Rock, Rps::Scissors) => 0,
        (Rps::Paper, Rps::Rock) => 0,
        (Rps::Paper, Rps::Scissors) => 6,
        (Rps::Scissors, Rps::Rock) => 6,
        (Rps::Scissors, Rps::Paper) => 0,
        _ => 3,
    }
}

fn play(theirs: Rps, ours: Rps) -> u32 {
    let round_score = score(theirs, ours);
    let round_bonus = match ours {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    };
    round_score + round_bonus
}

fn move_for_strategy(opp: Rps, strategy: char) -> Rps {
    match (opp, strategy) {
        (Rps::Rock, 'X') => Rps::Scissors,
        (Rps::Rock, 'Y') => Rps::Rock,
        (Rps::Rock, 'Z') => Rps::Paper,
        (Rps::Scissors, 'X') => Rps::Paper,
        (Rps::Scissors, 'Y') => Rps::Scissors,
        (Rps::Scissors, 'Z') => Rps::Rock,
        (Rps::Paper, 'X') => Rps::Rock,
        (Rps::Paper, 'Y') => Rps::Paper,
        (Rps::Paper, 'Z') => Rps::Scissors,
        _ => panic!(),
    }
}

pub fn day_2() -> (String, String) {
    let f = read_to_string("input/day2.txt").unwrap();

    let mut total_a = 0;
    let mut total_b = 0;

    for l in f.lines() {
        if !l.is_empty() {
            let theirs = parse_rps(l.chars().next().unwrap());
            let ours = parse_rps(l.chars().nth(2).unwrap());
            let ours_2 = move_for_strategy(theirs, l.chars().nth(2).unwrap());

            total_a += play(theirs, ours);
            total_b += play(theirs, ours_2);
        }
    }

    (format!("{}", total_a), format!("{}", total_b))
}
