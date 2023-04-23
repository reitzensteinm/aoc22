use std::fs::read_to_string;

#[derive(Copy, Clone, Debug)]
struct CodeItem {
    val: i16,
    pos: u16,
}

#[derive(Debug, Clone)]
struct CodeBucket {
    items: Vec<CodeItem>,
}

#[derive(Debug, Clone)]
struct Code {
    multiplier: isize,
    buckets: Vec<CodeBucket>,
    index: Vec<usize>,
    length: usize,
}

impl Code {
    fn to_array(&self) -> Vec<i16> {
        let mut out = vec![];
        for cb in &self.buckets {
            for i in &cb.items {
                out.push(i.val);
            }
        }
        out
    }

    fn process(&mut self, insertion_order: usize) {
        let bucket = self.index[insertion_order];

        let mut pos = 0;
        for c in 0..bucket {
            pos += self.buckets[c].items.len();
        }

        for i in 0..self.buckets[bucket].items.len() {
            if self.buckets[bucket].items[i].pos == insertion_order as u16 {
                let ci = self.buckets[bucket].items.remove(i);
                let mut slot = target_slot(pos, ci.val as isize * self.multiplier, self.length - 1);

                for b in 0..self.buckets.len() {
                    if slot <= self.buckets[b].items.len() {
                        self.buckets[b].items.insert(slot, ci);
                        self.index[insertion_order] = b;
                        return;
                    }
                    slot -= self.buckets[b].items.len();
                }
                panic!();
            }
            pos += 1;
        }

        panic!();
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

fn answer(arr: Vec<i16>, multiplier: isize) -> isize {
    for (i, v) in arr.iter().enumerate() {
        if *v == 0 {
            let g = |ind: usize| arr[ind % arr.len()] as isize * multiplier;
            return (g(i + 1000) + g(i + 2000) + g(i + 3000)) as isize;
        }
    }
    panic!()
}

pub fn day_20_new() -> (String, String) {
    let f = read_to_string("input/day20.txt").unwrap();

    let input = f
        .lines()
        .map(|l| str::parse::<i16>(l).unwrap())
        .collect::<Vec<i16>>();

    let bucket_size = 100;
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

        buckets.push(CodeBucket { items });
    }

    let mut code = Code {
        buckets,
        multiplier: 1,
        index,
        length: input.len(),
    };

    let mut code_b = code.clone();
    code_b.multiplier = 811589153;

    for n in 0..5000 {
        code.process(n);
    }

    for _ in 0..10 {
        for n in 0..5000 {
            code_b.process(n);
        }
    }

    let arr = code.to_array();
    let out_a = answer(arr, 1).to_string();

    let arrb = code_b.to_array();
    let out_b = answer(arrb, code_b.multiplier).to_string();

    (out_a, out_b)
}
