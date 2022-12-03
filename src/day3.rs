use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Item {
    code: char,
}

impl Item {
    fn make_from_code(code: char) -> Result<Self,String> {
        if !code.is_ascii_alphabetic(){
            Err(String::from("Character is not alphabetic"))
        }
        else{
            Ok(Item {code})
        }
    }

    fn priority(&self) -> u32 {
        if self.code.is_lowercase() {
            self.code as u32 - 'a' as u32 + 1
        }
        else {
            self.code as u32 - 'A' as u32 + 27
        }
    }
}

pub fn day3() {
    let mut file = File::open("./inputs/input_day3.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let sum_priorities_comp = data.split('\n')
        .map(|rucksack_str| {
            let (comp1_str,comp2_str) = rucksack_str.split_at(rucksack_str.len() / 2);
            let comp1: HashSet<Item> = comp1_str.chars().map(|code| Item::make_from_code(code).unwrap()).collect();
            let comp2: HashSet<Item> = comp2_str.chars().map(|code| Item::make_from_code(code).unwrap()).collect();
            comp1.intersection(&comp2).next().unwrap().priority()
        })
        .sum::<u32>();

    let sum_priorities_badge = data.split('\n')
        .map(|rucksack_str| rucksack_str.chars().map(|code| Item::make_from_code(code).unwrap()).collect::<HashSet<Item>>())
        .chunks(3).into_iter()
        .map(|chunk_three| chunk_three.reduce(|accum, item|
            accum.intersection(&item).copied().collect()).unwrap().into_iter().next().unwrap().priority())
        .sum::<u32>();

    println!("Solution 1 : {:?}", sum_priorities_comp);
    println!("Solution 2 : {:?}", sum_priorities_badge);
}
