use std::fs::read_to_string;

fn snafu_to_num(s: &str) -> isize {
    let mut out = 0;
    for c in s.chars() {
        out *= 5;
        out += match c {
            '1' => 1,
            '2' => 2,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        }
    }
    out
}

fn num_to_snafu(mut n: isize) -> String {
    let mut out: String = "".to_string();
    loop {
        let (s, off) = match n % 5 {
            0 => ("0", 0),
            1 => ("1", 1),
            2 => ("2", 2),
            3 => ("=", -2),
            4 => ("-", -1),
            _ => panic!(),
        };

        n -= off;
        out = s.to_string() + &out;
        n /= 5;

        if n == 0 {
            return out;
        }
    }
}

pub fn day_25() -> (String, String) {
    let f = read_to_string("input/day25.txt").unwrap();

    let mut sum = 0;
    for l in f.lines() {
        assert_eq!(num_to_snafu(snafu_to_num(l)), l);
        sum += snafu_to_num(l);
    }

    (num_to_snafu(sum), "yay".to_string())
}
