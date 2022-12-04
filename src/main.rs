use crate::days::day1::day_1;
use crate::days::day2::day_2;
use crate::days::day3::day_3;
use std::time;

mod days;

fn main() {
    println!("Hello, world!");

    let start = time::Instant::now();
    let run_day = |d: &dyn Fn() -> (String, String)| {
        println!("{:?}", d());
        println!("{}", (time::Instant::now() - start).as_millis());
    };

    run_day(&day_1);
    run_day(&day_2);
    run_day(&day_3);
}
