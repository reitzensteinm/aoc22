use std::fs::read_to_string;

// This is a fairly unambitious solution that simply stores the numbers in a Vec, slicing
// it up as needed. This is n^2, but n is small, so it performs ~250 million operations
// There are ways to speed it up, but at ~20ms, that's probably out of scope for this run.

#[derive(Copy, Clone)]
struct CodeItem {
    val: i16,
    pos: u16,
}

struct Code {
    multiplier: isize,
    items: Vec<CodeItem>,
}

impl Code {
    fn from_vec(vec: &[i16]) -> Code {
        Code {
            multiplier: 1,
            items: vec
                .iter()
                .enumerate()
                .map(|(i, v)| CodeItem {
                    val: *v,
                    pos: i as u16,
                })
                .collect(),
        }
    }

    #[cfg(test)]
    fn as_array(&self) -> Vec<isize> {
        self.items
            .iter()
            .map(|v| v.val as isize * self.multiplier)
            .collect::<Vec<isize>>()
    }

    // This is n^2
    fn decode(&mut self) {
        let mut ind = vec![0; self.items.len()];

        for (p, v) in self.items.iter().enumerate() {
            ind[v.pos as usize] = p;
        }

        let process = |items: &mut Vec<CodeItem>, multiplier: isize, n: isize, i: usize| -> bool {
            if n >= 0 && n < items.len() as isize {
                if items[n as usize].pos == i as u16 {
                    let i = items.remove(n as usize);
                    let loc = target_slot(n as usize, i.val as isize * multiplier, items.len());
                    items.insert(loc, i);
                    return true;
                }
            }
            false
        };

        let mut miss_up = 0;
        let mut miss_down = 0;

        for i in 0..self.items.len() {
            // At the start, there's a 50% chance an element will be shifted to later in the array
            // This means that the element we're searching for is likely to be earlier by n / 2
            let mut expected: isize = ind[i] as isize - i as isize / 2;

            // The code reaches a steady state, and elements aren't shifted around much. This
            // is a circuit breaker against that eventuality
            if miss_up > miss_down * 2 {
                expected = ind[i] as isize;
            }

            for n in 0.. {
                if process(&mut self.items, self.multiplier, expected + n, i) {
                    miss_up += n;
                    break;
                }

                if process(&mut self.items, self.multiplier, expected - (n + 1), i) {
                    miss_down += n;
                    break;
                }
            }
        }
    }

    fn answer(&self) -> isize {
        for (i, v) in self.items.iter().enumerate() {
            if v.val == 0 {
                let g =
                    |ind: usize| self.items[ind % self.items.len()].val as isize * self.multiplier;
                return (g(i + 1000) + g(i + 2000) + g(i + 3000)) as isize;
            }
        }
        panic!()
    }
}

fn target_slot(current: usize, offset: isize, length: usize) -> usize {
    let loci = current as isize + offset;
    let leni = length as isize;
    (if offset == 0 {
        loci
    } else if loci <= 0 {
        leni - (leni - loci) % leni
    } else if loci >= leni {
        loci % leni
    } else {
        loci
    }) as usize
}

pub fn day_20() -> (String, String) {
    let f = read_to_string("input/day20.txt").unwrap();
    let input = f
        .lines()
        .map(|l| str::parse::<i16>(l).unwrap())
        .collect::<Vec<i16>>();

    let mut code_a = Code::from_vec(&input);
    code_a.decode();

    let mut code_b = Code::from_vec(&input);
    code_b.multiplier = 811589153;

    for _ in 0..10 {
        code_b.decode();
    }

    (
        format!("{}", code_a.answer()),
        format!("{}", code_b.answer()),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode() {
        let v = vec![1, 2, -3, 3, -2, 0, 4];
        let mut c = Code::from_vec(&v);
        c.decode();
        assert_eq!(c.as_array(), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    fn target_slot_old(current: usize, offset: isize, length: usize) -> usize {
        let mut loci = current as isize + offset;
        // negative mod math makes my head hurt
        if loci <= 0 {
            while loci <= 0 {
                loci += length as isize;
            }
        } else if loci >= length as isize {
            while loci >= length as isize {
                loci -= length as isize;
            }
        }

        loci as usize
    }
    #[test]
    fn test_mod() {
        for x in 0..100 {
            for y in 0..100 {
                for z in 5..50 {
                    assert_eq!(target_slot(x, y, z), target_slot_old(x, y, z));
                }
            }
        }
    }
}
