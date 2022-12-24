use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use pathfinding::prelude::{astar};

const ALL_DIRECTIONS: [Direction;5] = [Direction::Up,Direction::Down,Direction::Left,Direction::Right,Direction::None];

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn get_movement(&self) -> (i32,i32) {
        match self {
            Direction::Up => (0,-1),
            Direction::Down => (0,1),
            Direction::Left => (-1,0),
            Direction::Right => (1,0),
            Direction::None => (0,0)
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Blizzard {
    fn pos_n_turn(&self, width: i32, height: i32, nb_turn : usize) -> (i32,i32) {
        let (dx,dy) = self.direction.get_movement();
        (((self.x + (nb_turn as i32) * dx - 1).rem_euclid(width)) + 1, ((self.y + (nb_turn as i32) * dy - 1).rem_euclid(height)) + 1)
    }

    fn parse(x: i32, y: i32, c: char) -> Option<Self> {
        match c {
            '^' => Some(Blizzard {x,y, direction: Direction::Up}),
            '>' => Some(Blizzard {x,y, direction: Direction::Right}),
            'v' => Some(Blizzard {x,y, direction: Direction::Down}),
            '<' => Some(Blizzard {x,y, direction: Direction::Left}),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    time: usize,
    x: i32,
    y: i32,
}

impl Node {
    fn get_neighbors(&self, width: i32, height: i32, blizzard_pos_next_turns: HashSet<(i32,i32)>) -> Vec<(Node,usize)> {
        ALL_DIRECTIONS.iter()
            .map(|dir| {
                let (dx,dy) = dir.get_movement();
                (self.x + dx,self.y + dy)
            })
            .filter(|(u,v)| ((*u == 1 && *v == 0)
                 || (*u == width && *v == height + 1)
                 || (*u > 0 && *v > 0 && *u <= width && *v <= height))
                && !blizzard_pos_next_turns.contains(&(*u,*v)))
            .map(|(u,v)| (Node {
                time: self.time + 1,
                x: u,
                y: v,
            },1))
            .collect()
    }

    fn get_time_travel(start_node: &Node, end_x: i32, end_y: i32, width: i32, height: i32, blizzards: &[Blizzard]) -> usize {
        astar(start_node,
              |node| node.get_neighbors(width,height,blizzards.iter()
                  .map(|blizzard| blizzard.pos_n_turn(width,height,node.time + 1))
                  .collect()),
              |node| ((node.x - end_x).abs() + (node.y - end_y).abs()) as usize,
              |node| node.x == end_x && node.y == end_y)
            .unwrap().1
    }
}
pub fn day24() {
    let mut file = File::open("./inputs/input_day24.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    // height and width of available space
    let height: i32 = (data.lines().count() - 2) as i32;
    let width: i32 = (data.lines().next().unwrap().len() - 2) as i32;

    let start = (1,0);
    let end = (width, height+1);

    let blizzards = data.lines()
        .enumerate()
        .map(|(y,s)| s.chars()
            .enumerate()
            .filter_map(|(x,c)| Blizzard::parse(x as i32,y as i32,c))
            .collect::<Vec<Blizzard>>())
        .concat();

    let start_node1 = Node {
        time: 0,
        x: start.0,
        y: start.1,
    };
    let time1 = Node::get_time_travel(&start_node1,end.0,end.1,width,height,&blizzards);

    println!("Solution 1 : {:?}",time1);

    let end_node = Node {
        time: time1,
        x: end.0,
        y: end.1,
    };
    let time2 = Node::get_time_travel(&end_node,start.0,start.1,width,height,&blizzards);

    let start_node2 = Node {
        time: time1 + time2,
        x: start.0,
        y: start.1,
    };
    let time3 = Node::get_time_travel(&start_node2,end.0,end.1,width,height,&blizzards);

    println!("Solution 2 : {:?}",time1 + time2 + time3);
}