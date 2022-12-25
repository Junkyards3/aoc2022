use std::fs::File;
use std::io::Read;

fn snafu_to_i64(s: &str) -> i64 {
    let mut result = 0;
    s.chars()
        .for_each(|c| {
            result = 5 * result + match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!()
            }
        });
    result
}

fn i64_to_snafu(v : i64) -> String {
    let mut result = String::from("");
    let mut number = v;
    while number != 0 {
        let rem = ((number+2) % 5) - 2;
        number = number / 5 + i64::from(rem < 0);
        result.insert(0, match rem {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!()
        })
    }
    result
}

pub fn day25() {
    let mut file = File::open("./inputs/input_day25.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let sol1 = i64_to_snafu(data.lines()
        .map(snafu_to_i64)
        .sum::<i64>());

    println!("Solution 1 : {}",sol1);
}