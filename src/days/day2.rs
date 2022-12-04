use std::fs::read_to_string;

#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn parse_rps(c: char) -> RPS {
    match c {
        'A' | 'X' => RPS::Rock,
        'B' | 'Y' => RPS::Paper,
        'C' | 'Z' => RPS::Scissors,
        _ => panic!(),
    }
}

fn score(opp: RPS, us: RPS) -> u32 {
    match (opp, us) {
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Scissors, RPS::Paper) => 0,
        _ => 3,
    }
}

fn play(theirs: RPS, ours: RPS) -> u32 {
    let round_score = score(theirs, ours);
    let round_bonus = match ours {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    round_score + round_bonus
}

fn move_for_strategy(opp: RPS, strategy: char) -> RPS {
    match (opp, strategy) {
        (RPS::Rock, 'X') => RPS::Scissors,
        (RPS::Rock, 'Y') => RPS::Rock,
        (RPS::Rock, 'Z') => RPS::Paper,
        (RPS::Scissors, 'X') => RPS::Paper,
        (RPS::Scissors, 'Y') => RPS::Scissors,
        (RPS::Scissors, 'Z') => RPS::Rock,
        (RPS::Paper, 'X') => RPS::Rock,
        (RPS::Paper, 'Y') => RPS::Paper,
        (RPS::Paper, 'Z') => RPS::Scissors,
        _ => panic!(),
    }
}

pub fn day_2() -> (String, String) {
    let f = read_to_string("input/day2.txt").unwrap();

    let mut total_a = 0;
    let mut total_b = 0;

    for l in f.split("\n") {
        if l.len() > 0 {
            let theirs = parse_rps(l.chars().nth(0).unwrap());
            let ours = parse_rps(l.chars().nth(2).unwrap());
            let ours_2 = move_for_strategy(theirs, l.chars().nth(2).unwrap());

            total_a += play(theirs, ours);
            total_b += play(theirs, ours_2);
        }
    }

    (format!("{}", total_a), format!("{}", total_b))
}
