use crate::days::day1::day_1;
use crate::days::day10::day_10;
use crate::days::day11::day_11;
use crate::days::day12::day_12;
use crate::days::day13::day_13;
use crate::days::day14::day_14;
use crate::days::day15::day_15;
use crate::days::day16::day_16;
use crate::days::day17::day_17;
use crate::days::day18::day_18;
use crate::days::day19::day_19;
use crate::days::day2::day_2;
use crate::days::day20::day_20;
use crate::days::day21::day_21;
use crate::days::day22::day_22;
use crate::days::day23::day_23;
use crate::days::day24::day_24;
use crate::days::day25::day_25;
use crate::days::day3::day_3;
use crate::days::day4::day_4;
use crate::days::day5::day_5;
use crate::days::day6::day_6;
use crate::days::day7::day_7;
use crate::days::day8::day_8;
use crate::days::day9::day_9;

use std::time;

mod days;
mod utils;

fn main() {
    let iters = 10;

    for i in 0..iters {
        let start = time::Instant::now();
        let mut last = start;
        let mut run_day = |d: &dyn Fn() -> (String, String)| {
            if i == iters - 1 {
                println!("{:?}", d());
                let now = time::Instant::now();
                println!(
                    "{} ({})",
                    (now - last).as_micros(),
                    (now - start).as_micros()
                );
                last = now;
            } else {
                d();
            }
        };

        run_day(&day_1);
        run_day(&day_2);
        run_day(&day_3);
        run_day(&day_4);
        run_day(&day_5);
        run_day(&day_6);
        run_day(&day_7);
        run_day(&day_8);
        run_day(&day_9);
        run_day(&day_10);
        run_day(&day_11);
        run_day(&day_12);
        run_day(&day_13);
        run_day(&day_14);
        run_day(&day_15);
        run_day(&day_16);
        run_day(&day_17);
        run_day(&day_18);
        run_day(&day_19);
        run_day(&day_20);
        run_day(&day_21);
        run_day(&day_22);
        run_day(&day_23);
        run_day(&day_24);
        run_day(&day_25);
    }
}
