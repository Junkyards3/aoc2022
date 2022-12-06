use std::fs::File;
use std::io::Read;

pub fn day6() {
    let mut file = File::open("./inputs/input_day6.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let binding = data.chars().collect::<Vec<char>>();
    let index = binding
        .windows(4)
        .enumerate()
        .find(|(_i,v)| {
            for i in 0..4 {
                for j in 0..i {
                    if v[i] == v[j]{
                        return false;
                    }
                }
            }
            true
        })
        .unwrap();
    let index2 = binding
        .windows(14)
        .enumerate()
        .find(|(_i,v)| {
            for i in 0..14 {
                for j in 0..i {
                    if v[i] == v[j]{
                        return false;
                    }
                }
            }
            true
        })
        .unwrap();
    println!("Solution 1 : {:?}", index.0 + 4);
    println!("Solution 2 : {:?}", index2.0 + 14);
}