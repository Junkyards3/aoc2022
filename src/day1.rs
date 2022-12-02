use std::fs::File;
use std::io::Read;

pub fn day1() {
    let mut file = File::open("./inputs/input_day1.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let mut calories_per_elf: Vec<u32> = data
        .split("\n\n")
        .map(|elf_str| {
            elf_str
                .split('\n')
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    calories_per_elf.sort_unstable();
    println!("Solution 1 : {:?}", calories_per_elf.iter().last().unwrap());
    println!(
        "Solution 2 : {:?}",
        calories_per_elf.iter().rev().take(3).sum::<u32>()
    );
}
