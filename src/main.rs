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
use crate::day18::day18;
use crate::day19::day19;

use crate::day1::day1;
use crate::day20::day20;
use crate::day21::day21;
use crate::day22::day22;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::day4;
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::day7;
use crate::day8::day8;
use crate::day9::day9;

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
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

fn time_function(f : fn() -> ()) {
    let now = Instant::now();
    f();
    let elapsed_time = now.elapsed();
    let elapsed_time_unit = elapsed_time.as_micros().to_string();
    println!("It took {} microseconds to run the function !", elapsed_time_unit)
}

fn main() {
println!("Choose your day !");
    let mut day_choice = String::new();

    io::stdin()
        .read_line(&mut day_choice)
        .expect("Failed to read line");

    let day_choice_nb = day_choice.trim().parse::<usize>().expect("Invalid number day ! ");
    match day_choice_nb {
        1 => {time_function(day1)}
        2 => {time_function(day2)}
        3 => {time_function(day3)}
        4 => {time_function(day4)}
        5 => {time_function(day5)}
        6 => {time_function(day6)}
        7 => {time_function(day7)}
        8 => {time_function(day8)}
        9 => {time_function(day9)}
        10 => {time_function(day10)}
        11 => {time_function(day11)}
        12 => {time_function(day12)}
        13 => {time_function(day13)}
        14 => {time_function(day14)}
        15 => {time_function(day15)}
        16 => {time_function(day16)}
        17 => {time_function(day17)}
        18 => {time_function(day18)}
        19 => {time_function(day19)}
        20 => {time_function(day20)}
        21 => {time_function(day21)}
        22 => {time_function(day22)}
        _ => {println!("No day corresponding to this number")}
    }
}
