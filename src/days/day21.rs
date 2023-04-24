use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Deref;

// In order to solve Part 2, values can be known, unknown (the input const), or partial evaluations
#[derive(Debug, Clone)]
enum Value {
    Known(isize),
    Unknown,
    Partial(Op, Box<Value>, Box<Value>),
}

#[derive(Debug, Clone)]
enum Op {
    KnownConst(isize),
    UnknownConst,
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Equal(String, String),
}

impl Op {
    fn parse(s: &str) -> (String, Op) {
        let (name, rest) = s.split_at(4);
        let (_, op) = rest.split_at(2);

        let op = if op.len() <= 4 {
            Op::KnownConst(str::parse::<isize>(op).unwrap())
        } else {
            let a = op[0..4].to_string();
            let b = op[7..11].to_string();
            let o = op.chars().nth(5).unwrap();

            if o == '+' {
                Op::Add(a, b)
            } else if o == '-' {
                Op::Subtract(a, b)
            } else if o == '*' {
                Op::Multiply(a, b)
            } else {
                Op::Divide(a, b)
            }
        };

        (name.to_string(), op)
    }
}

struct Machine {
    ops: HashMap<String, Op>,
}

impl Machine {
    fn execute_op(&self, scratch: &mut HashMap<String, Value>, name: &str) -> Value {
        if let Some(v) = scratch.get(name) {
            return v.clone();
        }

        let o = self.ops.get(name).unwrap();

        let mut value_op = |a, b, f: fn(isize, isize) -> isize| {
            let va = self.execute_op(scratch, a);
            let vb = self.execute_op(scratch, b);

            match (&va, &vb) {
                (Value::Known(ia), Value::Known(ib)) => Value::Known(f(*ia, *ib)),
                _ => Value::Partial(o.clone(), Box::new(va.clone()), Box::new(vb.clone())),
            }
        };

        let res = match o {
            Op::KnownConst(v) => Value::Known(*v),
            Op::UnknownConst => Value::Unknown,
            Op::Add(a, b) => value_op(a, b, |a, b| a + b),
            Op::Subtract(a, b) => value_op(a, b, |a, b| a - b),
            Op::Multiply(a, b) => value_op(a, b, |a, b| a * b),
            Op::Divide(a, b) => value_op(a, b, |a, b| a / b),
            Op::Equal(a, b) => value_op(a, b, |a, b| if a == b { 1 } else { 0 }),
        };

        scratch.insert(name.to_string(), res.clone());
        res
    }
    fn run(&self) -> Value {
        let mut scratch = HashMap::new();
        self.execute_op(&mut scratch, "root")
    }
}

// Simplifies partial expressions like:
// (a + 5) = 6
// -> a = 1
// Only what is required to complete the puzzle is implemented because the code is already pretty ugly.
fn simplify_equality(unknown: Value, known: Value) -> (Value, Value) {
    match (unknown, known) {
        (Value::Partial(Op::Divide(_, _), a, b), Value::Known(i)) => match (a.deref(), b.deref()) {
            (a, Value::Known(b)) => (a.clone(), Value::Known(i * b)),
            _ => panic!(),
        },
        (Value::Partial(Op::Add(_, _), a, b), Value::Known(i)) => match (a.deref(), b.deref()) {
            (a, Value::Known(b)) => (a.clone(), Value::Known(i - b)),
            (Value::Known(b), a) => (a.clone(), Value::Known(i - b)),
            _ => panic!(),
        },
        (Value::Partial(Op::Multiply(_, _), a, b), Value::Known(i)) => {
            match (a.deref(), b.deref()) {
                (a, Value::Known(b)) => (a.clone(), Value::Known(i / b)),
                (Value::Known(b), a) => (a.clone(), Value::Known(i / b)),
                _ => panic!(),
            }
        }
        (Value::Partial(Op::Subtract(_, _), a, b), Value::Known(i)) => {
            match (a.deref(), b.deref()) {
                (a, Value::Known(b)) => (a.clone(), Value::Known(i + b)),
                (Value::Known(a), b) => (b.clone(), Value::Known(a - i)),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn simplify_equality_loop(unknown: Value, known: Value) -> isize {
    let mut vals = (unknown, known);
    loop {
        vals = simplify_equality(vals.0, vals.1);
        if let Value::Known(i) = vals.1 {
            return i;
        }
    }
}

fn solve_equality(value: Value) -> isize {
    if let Value::Partial(_, a, b) = value {
        if matches!(*a.deref(), Value::Known(_)) {
            simplify_equality_loop(b.deref().clone(), a.deref().clone())
        } else {
            simplify_equality_loop(a.deref().clone(), b.deref().clone())
        }
    } else {
        panic!()
    }
}

pub fn day_21() -> (String, String) {
    let f = read_to_string("input/day21.txt").unwrap();

    let mut m = Machine {
        ops: HashMap::new(),
    };

    for l in f.lines() {
        let (name, op) = Op::parse(l);
        m.ops.insert(name, op);
    }

    // Part A
    let res = m.run();
    let part_a = match res {
        Value::Known(i) => i,
        _ => panic!(),
    };

    if let Op::Add(a, b) = m.ops.get("root").unwrap() {
        let new = Op::Equal(a.to_string(), b.to_string());
        m.ops.insert("root".to_string(), new);
    }

    m.ops.insert("humn".to_string(), Op::UnknownConst);

    // Get partial result out of machine
    let res = m.run();
    // then solve for the generated equation
    let eq = solve_equality(res);

    (format!("{}", part_a), format!("{}", eq))
}
