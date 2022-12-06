use std::fs::read_to_string;

pub fn day_6() -> (String, String) {
    let f = read_to_string("input/day6.txt").unwrap();

    let signal: Vec<char> = f.chars().collect();

    let check = |pos, size| {
        for x in 0..size {
            for ax in (x + 1)..size {
                if signal[x + pos] == signal[ax + pos] {
                    return false;
                }
            }
        }
        true
    };

    let mut packet_pos = None;
    let mut message_pos = None;

    for x in 0..signal.len() {
        if packet_pos.is_none() && check(x, 4) {
            packet_pos = Some(x + 4);
        }
        if message_pos.is_none() && check(x, 14) {
            message_pos = Some(x + 14);
        }
    }

    (
        format!("{}", packet_pos.unwrap()),
        format!("{}", message_pos.unwrap()),
    )
}
