use crate::days::day1::day_1;
use crate::days::day2::day_2;
use crate::days::day3::day_3;
use crate::days::day4::day_4;
use crate::days::day5::day_5;
use crate::days::day6::day_6;
use crate::days::day7::day_7;
use crate::days::day8::day_8;
use std::time;

mod days;

fn main() {
    let iters = 100;

    for i in 0..iters {
        let start = time::Instant::now();
        let run_day = |d: &dyn Fn() -> (String, String)| {
            if i == iters - 1 {
                println!("{:?}", d());
                println!("{}", (time::Instant::now() - start).as_micros());
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
    }
}
