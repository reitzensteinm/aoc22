use std::fs::read_to_string;

#[derive(Copy, Clone, Debug)]
struct CodeItem {
    val: i16,
    pos: u16,
}

#[derive(Debug, Clone)]
struct Code {
    multiplier: isize,
    buckets: Vec<Vec<CodeItem>>,
    index: Vec<usize>,
    length: usize,
}

impl Code {
    fn to_array(&self) -> Vec<i16> {
        let mut out = vec![];
        for cb in &self.buckets {
            for i in cb {
                out.push(i.val);
            }
        }
        out
    }

    fn process(&mut self, insertion_order: usize) {
        let bucket = self.index[insertion_order];

        let mut pos = 0;
        for c in 0..bucket {
            pos += self.buckets[c].len();
        }

        for i in 0..self.buckets[bucket].len() {
            if self.buckets[bucket][i].pos == insertion_order as u16 {
                let ci = self.buckets[bucket].remove(i);
                let mut slot = target_slot(pos, ci.val as isize * self.multiplier, self.length - 1);

                for b in 0..self.buckets.len() {
                    if slot <= self.buckets[b].len() {
                        self.buckets[b].insert(slot, ci);
                        self.index[insertion_order] = b;
                        return;
                    }
                    slot -= self.buckets[b].len();
                }

                panic!();
            }
            pos += 1;
        }

        panic!();
    }

    fn decode(&mut self) {
        for n in 0..self.length {
            self.process(n);
        }
    }

    fn answer(&self) -> isize {
        let arr = self.to_array();
        for (i, v) in arr.iter().enumerate() {
            if *v == 0 {
                let g = |ind: usize| arr[ind % arr.len()] as isize * self.multiplier;
                return g(i + 1000) + g(i + 2000) + g(i + 3000);
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

    // sqrt(5000) is probably a good guess here
    let bucket_size = 50;
    assert_eq!(input.len() % bucket_size, 0);

    let mut buckets = vec![];
    let mut index = vec![];

    for x in 0..input.len() / bucket_size {
        let mut items = vec![];

        for c in 0..bucket_size {
            let i = x * bucket_size + c;
            items.push(CodeItem {
                val: input[i],
                pos: i as u16,
            });
            index.push(x);
        }

        buckets.push(items);
    }

    let mut code_a = Code {
        buckets,
        multiplier: 1,
        index,
        length: input.len(),
    };

    let mut code_b = code_a.clone();
    code_b.multiplier = 811589153;

    code_a.decode();

    for _ in 0..10 {
        code_b.decode();
    }

    let out_a = code_a.answer().to_string();
    let out_b = code_b.answer().to_string();

    (out_a, out_b)
}
