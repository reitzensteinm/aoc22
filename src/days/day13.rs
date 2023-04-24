use std::cmp::Ordering;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

fn packet_compare(a: &Packet, b: &Packet) -> Ordering {
    match (a, b) {
        (Packet::Number(na), Packet::Number(nb)) => na.cmp(nb),
        (Packet::List(la), Packet::List(lb)) => {
            for n in 0.. {
                match (n >= la.len(), n >= lb.len()) {
                    (true, false) => return Ordering::Less,
                    (false, true) => return Ordering::Greater,
                    (true, true) => return Ordering::Equal,
                    (false, false) => {
                        let o = packet_compare(&la[n], &lb[n]);
                        if o != Ordering::Equal {
                            return o;
                        }
                    }
                }
            }
            // Rust doesn't know the above can't terminate?
            todo!()
        }
        (Packet::List(_), Packet::Number(n)) => {
            packet_compare(a, &Packet::List(vec![Packet::Number(*n)]))
        }
        (Packet::Number(n), Packet::List(_)) => {
            packet_compare(&Packet::List(vec![Packet::Number(*n)]), b)
        }
    }
}

struct Stream {
    chars: Vec<char>,
    index: usize,
}

impl Stream {
    fn eof(&self) -> bool {
        self.index >= self.chars.len()
    }
    fn peek(&self) -> char {
        self.chars[self.index]
    }
    fn read(&mut self) -> char {
        let res = self.peek();
        self.index += 1;
        res
    }
}

fn read_array(s: &mut Stream) -> Packet {
    s.read();
    let mut out = vec![];

    loop {
        // We're assuming properly formed input, here ¯\_(ツ)_/¯
        if s.peek() == ']' {
            s.read();
            return Packet::List(out);
        } else if s.peek() == ',' {
            s.read();
        } else {
            out.push(read(s));
        }
    }
}

fn read(s: &mut Stream) -> Packet {
    if s.peek() == '[' {
        read_array(s)
    } else {
        let mut nums = "".to_string();

        while !s.eof() && s.peek().is_ascii_digit() {
            nums.push(s.read());
        }

        Packet::Number(str::parse::<usize>(&nums).unwrap())
    }
}

fn parse(ps: &str) -> Packet {
    let mut s = Stream {
        chars: ps.chars().collect(),
        index: 0,
    };

    read(&mut s)
}

pub fn day_13() -> (String, String) {
    let f = read_to_string("input/day13.txt").unwrap();
    let lines: Vec<&str> = f.lines().collect();

    let mut equal_sum = 0;
    let mut packets = vec![];

    for (i, l) in lines.chunks(3).enumerate() {
        let packet_a = parse(l[0]);
        let packet_b = parse(l[1]);

        let ordering = packet_compare(&packet_a, &packet_b);

        if ordering == Ordering::Less {
            equal_sum += i + 1
        }

        packets.push(packet_a);
        packets.push(packet_b);
    }

    let divider_a = parse("[[2]]");
    let divider_b = parse("[[6]]");

    packets.push(divider_a.clone());
    packets.push(divider_b.clone());

    packets.sort_by(packet_compare);

    let mut divider_mult = 1;

    for (n, packet) in packets.iter().enumerate() {
        if packet_compare(packet, &divider_a) == Ordering::Equal
            || packet_compare(packet, &divider_b) == Ordering::Equal
        {
            divider_mult *= n + 1;
        }
    }

    (format!("{}", equal_sum), format!("{}", divider_mult))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn packet_test() {
        assert_eq!(parse("123"), Packet::Number(123));
        assert_eq!(parse("[]"), Packet::List(vec![]));
        assert_eq!(parse("[123]"), Packet::List(vec![Packet::Number(123)]));
        assert_eq!(
            parse("[[1],4]"),
            Packet::List(vec![
                Packet::List(vec![Packet::Number(1)]),
                Packet::Number(4)
            ])
        );
    }
}
