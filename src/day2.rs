use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win
}

#[derive(Debug)]
struct MovePair{
    your_move: Move,
    opponent_move: Move,
}

struct MoveOutcome {
    opponent_move: Move,
    outcome: Outcome
}

impl MoveOutcome {
    fn score_outcome(&self) -> u32 {
        match self.outcome {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn score_shape(&self) -> u32 {
        let your_move = match (self.opponent_move,self.outcome) {
            (Move::Scissors,Outcome::Loss) => Move::Paper,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Rock,Outcome::Loss) => Move::Scissors,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Paper,Outcome::Loss) => Move::Rock,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Paper, Outcome::Win) => Move::Scissors,
        };
        your_move.score_shape()
    }

    fn score_total(&self) -> u32 {
        self.score_outcome() + self.score_shape()
    }
}

impl MovePair{
    fn score_total(&self) -> u32 {
        self.your_move.score_total(&self.opponent_move)
    }
}

impl Move {
    fn score_outcome(&self, other_move: &Move) -> u32 {
        if self > other_move { 6 - other_move.score_outcome(self) }
        else if self == other_move { 3 }
        else{
            // in this case, we should have self < other_move
            match self {
                Move::Rock => if let Move::Paper = other_move { 0 } else{ 6 },
                Move::Paper => 0,
                _ => {panic!("The case {:?} < {:?} should not happen",self,other_move)}
            }
        }
    }

    fn score_shape(&self) -> u32 {
        (*self) as u32
    }

    fn score_total(&self, other_move: &Move) -> u32 {
        self.score_outcome(other_move) + self.score_shape()
    }
}


impl FromStr for MovePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters: Vec<&str> = s.split(" ").collect();
        let opponent_move = match letters[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(String::from("Cannot be parsed")),
        };
        let your_move = match letters[1] {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => return Err(String::from("Cannot be parsed")),
        };
        Ok(MovePair {your_move, opponent_move})
    }
}

impl FromStr for MoveOutcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters: Vec<&str> = s.split(" ").collect();
        let opponent_move = match letters[0] {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            _ => return Err(String::from("Cannot be parsed")),
        };
        let outcome = match letters[1] {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => return Err(String::from("Cannot be parsed")),
        };
        Ok(MoveOutcome { opponent_move, outcome})
    }
}

pub fn day2() {
    let mut file = File::open("./inputs/input_day2.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let total_score1: u32 = data
        .split('\n')
        .map(| str_round| MovePair::from_str(str_round).expect("failed to parse").score_total())
        .sum();
    println!("Solution 1 : {:?}",total_score1);
    let total_score2: u32 = data
        .split('\n')
        .map(| str_round| MoveOutcome::from_str(str_round).expect("failed to parse").score_total())
        .sum();
    println!("Solution 1 : {:?}",total_score2);
}
