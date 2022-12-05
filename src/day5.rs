use std::fs::File;
use std::io::Read;
use std::str::FromStr;

struct Cargo {
    crates: Vec<Vec<char>>
}

struct Move {
    origin: usize,
    destination: usize,
    number: usize
}

impl Cargo {
    fn make_move(&mut self, crates_move: &Move) {
        for _ in 1..=crates_move.number {
            let x = self.crates[crates_move.origin].pop().unwrap();
            self.crates[crates_move.destination].push(x)
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ').filter_map(|s| s.parse::<usize>().ok());
        let number = it.next().unwrap();
        let origin = it.next().unwrap();
        let destination = it.next().unwrap();
        Ok(Move {origin, destination, number})
    }
}

impl FromStr for Cargo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(' ');
        let nb_stacks = it
            .next_back()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .max()
            .unwrap();
        let mut crates: Vec<Vec<char>> = vec![Vec::new(); nb_stacks];
    }
}
pub fn day5() {
    let mut file = File::open("./inputs/input_day4.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

}