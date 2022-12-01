use std::fmt::{Display, Formatter};
use std::io;
use std::time::Instant;

use crate::day1::day1;
use crate::TimeUnit::Nanoseconds;

mod day1;

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

    let guess_n = guess.trim().parse::<usize>().expect("Invalid number day !");
    match guess_n {
        1 => {time_function(day1,Nanoseconds)}
        _ => {println!("No day corresponding to this number")}
    }
}
