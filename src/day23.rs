use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;


#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn get_movement(&self) -> (i32,i32) {
        match self {
            Direction::North => (0,1),
            Direction::South => (0,-1),
            Direction::West => (-1,0),
            Direction::East => (1,0)
        }
    }

    fn get_adjacent_considered(&self) -> Vec<AdjacentPosition> {
        match self {
            Direction::North => vec![AdjacentPosition::North,AdjacentPosition::NorthEast,AdjacentPosition::NorthWest],
            Direction::South => vec![AdjacentPosition::South,AdjacentPosition::SouthEast,AdjacentPosition::SouthWest],
            Direction::West => vec![AdjacentPosition::West,AdjacentPosition::NorthWest,AdjacentPosition::SouthWest],
            Direction::East => vec![AdjacentPosition::East,AdjacentPosition::NorthEast,AdjacentPosition::SouthEast],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum AdjacentPosition {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl AdjacentPosition {
    fn get_relative(&self) -> (i32,i32) {
        match self {
            AdjacentPosition::North => (0,1),
            AdjacentPosition::NorthEast => (1,1),
            AdjacentPosition::East => (1,0),
            AdjacentPosition::SouthEast => (1,-1),
            AdjacentPosition::South => (0,-1),
            AdjacentPosition::SouthWest => (-1,-1),
            AdjacentPosition::West => (-1,0),
            AdjacentPosition::NorthWest => (-1,1),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Elf {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Elves {
    elves: HashSet<Elf>,
    direction_start: usize,
}

impl Elves {
    fn new(elves: HashSet<Elf>) -> Self {
        Elves { elves, direction_start: 0 }
    }

    fn is_occupied(&self, x: i32, y: i32) -> bool {
        self.elves.iter()
            .any(|elf| elf.x == x && elf.y == y)
    }

    fn simulate_round(&mut self) -> bool {
        let direction_round = [DIRECTION_ORDER[(self.direction_start)%4],DIRECTION_ORDER[(self.direction_start+1)%4],
            DIRECTION_ORDER[(self.direction_start+2)%4],DIRECTION_ORDER[(self.direction_start+3)%4]];
        let mut chosen_destination = HashMap::new();
        let mut number_times_chosen = HashMap::new();
        //first half
        for elf in self.elves.iter() {
            let (x,y) = (elf.x,elf.y);
            let free_adjacent_positions = ADJACENT_POSITIONS.iter()
                .filter(|adj_pos| {
                    let (dx,dy) = adj_pos.get_relative();
                    !self.is_occupied(x+dx,y+dy)
                })
                .cloned()
                .collect::<HashSet<AdjacentPosition>>();
            if free_adjacent_positions.len() != 8 {
                for dir in direction_round.iter() {
                    let is_dir_chosen = dir.get_adjacent_considered().iter()
                        .all(|adj_pos| free_adjacent_positions.contains(adj_pos));
                    if is_dir_chosen {
                        let (dx, dy) = dir.get_movement();
                        let choice_pos = (x + dx, y + dy);
                        chosen_destination.insert(elf, choice_pos);
                        number_times_chosen.entry(choice_pos).and_modify(|nb_times| *nb_times += 1).or_insert(1);
                        break;
                    }
                }
            }
        }
        let mut new_elves = HashSet::new();
        let mut has_moved = false;
        //second half
        for elf in self.elves.iter() {
            if let Some(choice_pos) = chosen_destination.get(elf) {
                if number_times_chosen[choice_pos] == 1 {
                    new_elves.insert(Elf {
                        x: choice_pos.0,
                        y: choice_pos.1,
                    });
                    has_moved = true;
                }
                else {
                    new_elves.insert(elf.clone());
                }
            }
            else {
                new_elves.insert(elf.clone());
            }
        }
        self.elves = new_elves;
        self.direction_start = (self.direction_start + 1);
        has_moved
    }

    fn get_empty_ground(&self) -> i32 {
        let any_elf = self.elves.iter().next().unwrap();
        let (x,y) = (any_elf.x,any_elf.y);
        let mut north = y;
        let mut south = y;
        let mut east = x;
        let mut west = x;
        for elf in self.elves.iter() {
            let (u,v) = (elf.x,elf.y);
            north = north.max(v);
            south = south.min(v);
            east = east.max(u);
            west = west.min(u);
        }
        (north - south + 1) * (east - west + 1) - (self.elves.len() as i32)
    }

    fn draw(&self) {
        let any_elf = self.elves.iter().next().unwrap();
        let (x,y) = (any_elf.x,any_elf.y);
        let mut north = y;
        let mut south = y;
        let mut east = x;
        let mut west = x;
        for elf in self.elves.iter() {
            let (u,v) = (elf.x,elf.y);
            north = north.max(v);
            south = south.min(v);
            east = east.max(u);
            west = west.min(u);
        }
        for j in south..=north {
            let mut s = String::new();
            for i in west..=east {
                let to_push = match self.is_occupied(i,north - j + south) {
                    true => '#',
                    false => '.'
                };
                s.push(to_push);
            }
            s.push(' ');
            s.push_str(&j.to_string());
            if j == north {
                s.push('\n');
                s.push_str(&west.to_string());
            }
            println!("{s}");
        }
        println!();
    }
}

const DIRECTION_ORDER: [Direction; 4] = [Direction::North,Direction::South,Direction::West,Direction::East];
const ADJACENT_POSITIONS: [AdjacentPosition; 8] = [AdjacentPosition::North,AdjacentPosition::East,AdjacentPosition::South,
    AdjacentPosition::West ,AdjacentPosition::NorthWest,AdjacentPosition::NorthEast,AdjacentPosition::SouthWest,
    AdjacentPosition::SouthEast];

pub fn day23() {
    let mut file = File::open("./inputs/input_day23.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut elves_set = HashSet::new();
    data.lines()
        .rev()
        .enumerate()
        .for_each(|(i,l)| {
            l.chars()
                .enumerate()
                .filter(|(_,c)| *c == '#')
                .for_each(|(j,_)| { elves_set.insert(Elf { x: j as i32, y: i as i32 }); });
        });
    let mut elves = Elves::new(elves_set);

    loop {
        let has_moved = elves.simulate_round();
        if elves.direction_start == 10 {
            println!("Solution 1 : {}",elves.get_empty_ground());
        }
        if !has_moved {
            break;
        }
        println!("Round number {}",elves.direction_start);
    }

    println!("Solution 2 : {}",elves.direction_start);
}