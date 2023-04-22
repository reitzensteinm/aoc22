use crate::utils::bit_grid::{BitGrid, Or, Prim, Shifted};
use std::fs::read_to_string;
use std::time;

pub fn iter(bg: &mut BitGrid) {
    bg.print();

    let start = time::Instant::now();

    let mut live = bg.clone();

    let a0 = Shifted::new(bg, -1, 1);
    let a1 = Shifted::new(bg, 0, 1);
    let a2 = Shifted::new(bg, 1, 1);

    let a_move = Or::new(&a0, &a1);
    let a_move = Or::new(&a_move, &a2);
    let a_move = BitGrid::from_view(&a_move);

    let b0 = Shifted::new(bg, -1, -1);
    let b1 = Shifted::new(bg, 0, -1);
    let b2 = Shifted::new(bg, 1, -1);

    let b_move = Or::new(&b0, &b1);
    let b_move = Or::new(&b_move, &b2);
    let b_move = BitGrid::from_view(&b_move);

    let c0 = Shifted::new(bg, 1, -1);
    let c1 = Shifted::new(bg, 1, 0);
    let c2 = Shifted::new(bg, 1, 1);

    let c_move = Or::new(&c0, &c1);
    let c_move = Or::new(&c_move, &c2);
    let c_move = BitGrid::from_view(&c_move);

    let d0 = Shifted::new(bg, -1, -1);
    let d1 = Shifted::new(bg, -1, 0);
    let d2 = Shifted::new(bg, -1, 1);

    let d_move = Or::new(&d0, &d1);
    let d_move = Or::new(&d_move, &d2);
    let d_move = BitGrid::from_view(&d_move);

    let any = Or::new(&a_move, &b_move);
    let any = Or::new(&any, &c_move);
    let any = Or::new(&any, &d_move);
    let any = BitGrid::from_view(&any);

    let live = Prim::new(&live, &any, |a, b| a & b);

    // Phase 2

    let a_move = Prim::new(&a_move, &live, |a, b| !a & b);
    let live = &Prim::new(&live, &a_move, |a, b| a & !b);
    let a_move = Shifted::new(&a_move, 0, -1);
    let a_move = BitGrid::from_view(&a_move);

    let b_move = Prim::new(&b_move, live, |a, b| !a & b);
    let live = &Prim::new(live, &b_move, |a, b| a & !b);
    let b_move = Shifted::new(&b_move, 0, 1);
    let b_move = BitGrid::from_view(&b_move);

    let c_move = Prim::new(&c_move, live, |a, b| !a & b);
    let live = &Prim::new(live, &c_move, |a, b| a & !b);
    let c_move = Shifted::new(&c_move, -1, 0);
    let c_move = BitGrid::from_view(&c_move);

    let d_move = Prim::new(&d_move, live, |a, b| !a & b);
    let live = &Prim::new(live, &d_move, |a, b| a & !b);
    let d_move = Shifted::new(&d_move, 1, 0);
    let d_move = BitGrid::from_view(&d_move);

    // Block

    let a_movef = Prim::new(&a_move, &b_move, |a, b| a & !b);
    let a_movef = Prim::new(&a_movef, &c_move, |a, b| a & !b);
    let a_movef = Prim::new(&a_movef, &d_move, |a, b| a & !b);
    let a_movef = BitGrid::from_view(&a_movef);

    let b_movef = Prim::new(&b_move, &a_move, |a, b| a & !b);
    let b_movef = Prim::new(&b_movef, &c_move, |a, b| a & !b);
    let b_movef = Prim::new(&b_movef, &d_move, |a, b| a & !b);
    let b_movef = BitGrid::from_view(&b_movef);

    let c_movef = Prim::new(&c_move, &a_move, |a, b| a & !b);
    let c_movef = Prim::new(&c_movef, &b_move, |a, b| a & !b);
    let c_movef = Prim::new(&c_movef, &d_move, |a, b| a & !b);
    let c_movef = BitGrid::from_view(&c_movef);

    let d_movef = Prim::new(&d_move, &a_move, |a, b| a & !b);
    let d_movef = Prim::new(&d_movef, &b_move, |a, b| a & !b);
    let d_movef = Prim::new(&d_movef, &c_move, |a, b| a & !b);
    let d_movef = BitGrid::from_view(&d_movef);

    let res = Or::new(live, &a_movef);
    let res = Or::new(&res, &b_movef);
    let res = Or::new(&res, &c_movef);
    let res = Or::new(&res, &d_movef);

    let res = BitGrid::from_view(&res);

    let now = time::Instant::now();
    println!("{}", (now - start).as_micros());

    todo!();

    let am = BitGrid::from_view(&a_move);
    let bm = BitGrid::from_view(&b_move);
    let cm = BitGrid::from_view(&c_move);
    let dm = BitGrid::from_view(&d_move);
    let any = BitGrid::from_view(&any);

    println!();

    println!("am\n");
    am.print();
    println!("bm\n");
    bm.print();
    println!("cm\n");
    cm.print();
    println!("dm\n");
    dm.print();
    println!("any\n");
    any.print();
    println!("res\n");
    res.print();

    todo!();

    // let live = BitGrid::from_view(live);
    // live.print();
}

pub fn day_23() -> (String, String) {
    let f = read_to_string("input/day23.txt").unwrap();

    let ls: Vec<Vec<char>> = f.lines().map(|v| v.chars().collect()).collect();

    let width = ls[0].len();
    let height = ls.len();

    let mut bg = BitGrid::new(width, height);

    println!("{:?}", ls);

    println!("{} {}", width, height);

    for y in 0..height {
        for x in 0..width {
            if ls[y][x] == '#' {
                bg.set(x, y, true);
            }
        }
    }

    iter(&mut bg);

    todo!()
}
