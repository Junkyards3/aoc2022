use std::fmt::{Display, Formatter};
use std::io;
use std::time::Instant;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day13::day13;
use crate::day14::day14;
use crate::day15::day15;
use crate::day16::day16;
use crate::day17::day17;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;
use crate::TimeUnit::Microseconds;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

enum TimeUnit {
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let unit_str = match self {
            TimeUnit::Seconds => {"seconds"}
            TimeUnit::Milliseconds => {"milliseconds"}
            TimeUnit::Microseconds => {"microseconds"}
            TimeUnit::Nanoseconds => {"nanoseconds"}
        };
        write!(f,"{}",unit_str)
    }
}
fn time_function(f : fn() -> (), unit: TimeUnit) {
    let now = Instant::now();
    f();
    let elapsed_time = now.elapsed();
    let elapsed_time_unit = match unit {
        TimeUnit::Seconds => {elapsed_time.as_secs().to_string()}
        TimeUnit::Milliseconds => {elapsed_time.as_millis().to_string()}
        TimeUnit::Microseconds => {elapsed_time.as_micros().to_string()}
        TimeUnit::Nanoseconds => {elapsed_time.as_nanos().to_string()}
    };
    println!("It took {} {} to run the function !", elapsed_time_unit,unit)
}
fn main() {
println!("Choose your day !");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess_n = guess.trim().parse::<usize>().expect("Invalid number day ! ");
    match guess_n {
        1 => {time_function(day1,Microseconds)}
        2 => {time_function(day2,Microseconds)}
        3 => {time_function(day3,Microseconds)}
        4 => {time_function(day4,Microseconds)}
        5 => {time_function(day5,Microseconds)}
        6 => {time_function(day6,Microseconds)}
        7 => {time_function(day7,Microseconds)}
        8 => {time_function(day8,Microseconds)}
        9 => {time_function(day9,Microseconds)}
        10 => {time_function(day10,Microseconds)}
        11 => {time_function(day11,Microseconds)}
        12 => {time_function(day12,Microseconds)}
        13 => {time_function(day13,Microseconds)}
        14 => {time_function(day14,Microseconds)}
        15 => {time_function(day15,Microseconds)}
        16 => {time_function(day16,Microseconds)}
        17 => {time_function(day17,Microseconds)}
        _ => {println!("No day corresponding to this number")}
    }
}
