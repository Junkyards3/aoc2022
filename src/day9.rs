use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use ndarray::{Array2, Axis, Dimension};

#[derive(Debug)]
enum Relative { N, NE, E, SE, S, SW, W, NW, O }

impl Relative {
    fn displacement(&self) -> (i32, i32) {
        match self {
            Relative::N => (0, 1),
            Relative::NE => (1, 1),
            Relative::E => (1, 0),
            Relative::SE => (1, -1),
            Relative::S => (0, -1),
            Relative::SW => (-1, -1),
            Relative::W => (-1, 0),
            Relative::NW => (-1, 1),
            Relative::O => (0, 0)
        }
    }

    fn relative_after_move(&self, movet: &Move) -> Self {
        match (&self, move_head) {
            (Relative::N, Move::N) | (Relative::E, Move::E) |
            (Relative::S, Move::S) | (Relative::W, Move::W) => Relative::O,
            (Relative::N, Move::E) | (Relative::W, Move::S) => Relative::NW,
            (Relative::N, Move::S) | (Relative::NE, Move::E) | (Relative::NW, Move::S) |
            (Relative::NE, Move::S) | (Relative::NW, Move::W) | (Relative::O, Move::S) => Relative::N,
            (Relative::N, Move::W) | (Relative::E, Move::S) => Relative::NE,
            (Relative::NE, Move::N) | (Relative::NE, Move::W) | (Relative::E, Move::W) |
            (Relative::SE, Move::S) | (Relative::SE, Move::W) | (Relative::O, Move::W) => Relative::E,
            (Relative::E, Move::N) | (Relative::S, Move::W) => Relative::SE,
            (Relative::SE, Move::N) | (Relative::SE, Move::E) | (Relative::S, Move::N) |
            (Relative::SW, Move::N) | (Relative::SW, Move::W) | (Relative::O, Move::N) => Relative::S,
            (Relative::S, Move::E) | (Relative::W, Move::N) => Relative::SW,
            (Relative::SW, Move::E) | (Relative::SW, Move::S) | (Relative::W, Move::E) |
            (Relative::NW, Move::N) | (Relative::NW, Move::E) | (Relative::O, Move::E) => Relative::W,
        }
    }

    fn compute_position(&self, x: i32, y: i32) -> (i32,i32) {
        let displacement = self.displacement();
        (x + displacement.0, y + displacement.1)
    }
}

#[derive(Debug)]
enum Move { N, E, S, W }

impl Move {
    fn to_relative(&self) -> Relative {
        match self {
            Move::N => Relative::N,
            Move::E => Relative::E,
            Move::S => Relative::S,
            Move::W => Relative::W,
        }
    }
}

#[derive(Debug)]
struct Head {
    pos_x: i32,
    pos_y: i32,
}

#[derive(Debug)]
struct Tail {
    relative: Relative,
}

impl Default for Tail {
    fn default() -> Self {
        Tail { relative: Relative::O }
    }
}

#[derive(Debug)]
struct System {
    head: Head,
    tail: Tail,
    visited_tail_pos: HashSet<(i32, i32)>,
}

struct SystemMultipleTails {
    head: Head,
    tails: Vec<Tail>,
    visited_last_tail_pos: HashSet<(i32,i32)>
}

impl System {
    fn compute_tail_position(&self) -> (i32, i32) {
        let displacement = self.tail.relative.displacement();
        (self.head.pos_x + displacement.0, self.head.pos_y + displacement.1)
    }

    fn move_head(&mut self, move_head: &Move) {
        let displacement = move_head.to_relative().displacement();
        self.head.pos_x += displacement.0;
        self.head.pos_y += displacement.1;
        self.tail.relative = self.tail.relative.relative_after_move(move_head);
        self.visited_tail_pos.insert(self.compute_tail_position());
    }
}

impl SystemMultipleTails{
    fn compute_tails_pos(&mut self) -> Vec<(i32,i32)> {
        let mut curr_pos = (self.head.pos_x, self.head.pos_y);
        let mut retour = vec![];
        for tail in self.tails {
            curr_pos = tail.relative.compute_position(curr_pos.0,curr_pos.1);
            retour.push(curr_pos);
        }
        retour
    }
    fn move_head(&mut self, move_head: &Move) {
        let old_pos = self.compute_tails_pos();
        let displacement = move_head.to_relative().displacement();
        self.head.pos_x += displacement.0;
        self.head.pos_y += displacement.1;
        let mut curr_pos = (self.head.pos_x, self.head.pos_y);
        let mut movet =  move_head;
        for i in 0..self.tails.len() {
            let mut tail = &self.tails[i];
            let new_relative = tail.relative.relative_after_move(movet);
            //calculer le move de la tail en cours
            curr_pos = new_relative.compute_position(curr_pos.0,curr_pos.1);
            self.tails[i] = Tail { relative: new_relative };
        }
        self.visited_last_tail_pos.insert(self.compute_tail_position());
    }
}
pub fn day9() {
    let mut file = File::open("./inputs/input_day9.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut system = System {
        head: Head { pos_x: 0, pos_y: 0 },
        tail: Tail { relative: Relative::O },
        visited_tail_pos: HashSet::new(),
    };
    data.split('\n')
        .for_each(|s| {
            let mut i = s.split_whitespace();
            let movet = match i.next().unwrap() {
                "R" => Move::E,
                "U" => Move::N,
                "L" => Move::W,
                "D" => Move::S,
                _ => panic!(),
            };
            let nb_move = i.next().unwrap().parse::<u32>().unwrap();
            for k in 0..nb_move {
                system.move_head(&movet);
            }
        });
    println!("Solution 1 : {:?}", system.visited_tail_pos.len())
}