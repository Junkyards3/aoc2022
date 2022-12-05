use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Cargo {
    crates: Vec<Vec<char>>
}

#[derive(Debug)]
struct Move {
    origin: usize,
    destination: usize,
    number: usize
}

impl Cargo {
    fn make_move(&mut self, crates_move: &Move) {
        for _ in 1..=crates_move.number {
            let x = self.crates[crates_move.origin-1].pop().unwrap();
            self.crates[crates_move.destination-1].push(x)
        }
    }

    fn make_move_9001(&mut self, crates_move: &Move){
        let mut buffer = Vec::new();
        for _ in 1..=crates_move.number {
            let x = self.crates[crates_move.origin-1].pop().unwrap();
            buffer.push(x);
        }
        for _ in 1..=crates_move.number {
            let x = buffer.pop().unwrap();
            self.crates[crates_move.destination-1].push(x);
        }

    }
    fn get_message(&self) -> String {
        self.crates.iter()
            .map(|v| v.last().unwrap())
            .join("")
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
        let mut it = s.split('\n');
        let nb_stacks = it
            .next_back()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .max()
            .unwrap();
        let mut crates: Vec<Vec<char>> = vec![Vec::new(); nb_stacks];
        it.rev().for_each(|s| {
            let mut it_ligne = s.chars();
            it_ligne.next();
            it_ligne.step_by(4)
                .enumerate()
                .for_each(|(i,c)| {
                    if c.is_alphabetic() {
                        crates[i].push(c)
                    }
                });
        });
        Ok(Cargo {crates})
    }
}
pub fn day5() {
    let mut file = File::open("./inputs/input_day5.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let mut data_splited = data.split("\n\n");
    let mut cargo9000 = Cargo::from_str(data_splited.next().unwrap()).unwrap();
    let mut cargo9001 = cargo9000.clone();
    let moves = data_splited.next().unwrap()
        .split('\n')
        .map(|s| Move::from_str(s).unwrap())
        .for_each(|m| {
            cargo9000.make_move(&m);
            cargo9001.make_move_9001(&m)
        });
    println!("Solution 1 : {:?}", cargo9000.get_message());
    println!("Solution 2 : {:?}", cargo9001.get_message());
}