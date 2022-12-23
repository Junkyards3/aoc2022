use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Open,
    Close,
}

#[derive(Debug)]
enum Rotation {
    Clockwise,
    Counterclockwise
}

#[derive(Debug)]
enum Instruction {
    Turn(Rotation),
    Move(i32),
}

#[derive(Debug)]
struct Board {
    map: HashMap<(i32, i32), Tile>,
    x: i32,
    y: i32,
    facing: Facing,
    correspondence: HashMap<FaceSide,(FaceSide,Direction)>,
    cube_size: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum CubeRoll {
    Up,
    Right,
    Down,
    Left,
}

impl CubeRoll {
    fn get_disp(&self) -> (i32,i32){
        match self {
            CubeRoll::Up => (0,-1),
            CubeRoll::Right => (1,0),
            CubeRoll::Down => (0,1),
            CubeRoll::Left => (-1,0),
        }
    }
}

type Side = CubeRoll;


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct FaceSide {
    face: (i32, i32),
    side: Side,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Normal,
    Reverse,
}
//[0,x,y] is the bottom side, [0,0,0] the top left corner on bottom side, [0,0,1] the top right corner on bottom side
#[derive(Debug)]
struct Cube {
    vertices: [[[char;2];2];2]
}

impl Cube {
    fn new() -> Self {
        Cube { vertices: [ [['A','B'],['C','D']] , [['E','F'],['G','H']]  ] }
    }
    fn roll(&self, roll: &CubeRoll) -> Self {
        let mut new_vertices = [[['.';2];2];2];
        for ((x1,x2),x3) in (0..=1).cartesian_product(0..=1).cartesian_product(0..=1) {
            match roll {
                CubeRoll::Up => new_vertices[x2][1-x1][x3] = self.vertices[x1][x2][x3],
                CubeRoll::Right => new_vertices[1-x3][x2][x1] = self.vertices[x1][x2][x3],
                CubeRoll::Down => new_vertices[1-x2][x1][x3] = self.vertices[x1][x2][x3],
                CubeRoll::Left => new_vertices[x3][x2][1-x1] = self.vertices[x1][x2][x3]
            }
        }
        Cube { vertices: new_vertices}
    }
}

impl Facing {
    fn from_usize(disc: i32) -> Self {
        match disc {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => panic!(),
        }
    }
    fn rotate(&self, turn: &Rotation) -> Self {
        let rotation = match turn {
            Rotation::Clockwise => 1,
            Rotation::Counterclockwise => 3
        };
        Facing::from_usize((*self as i32 + rotation) % 4)
    }

    fn get_movement(&self) -> (i32, i32) {
        match self {
            Facing::Right => (1,0),
            Facing::Down => (0,1),
            Facing::Left => (-1,0),
            Facing::Up => (0,-1)
        }
    }
}

impl Board {
    fn parse(s: &str, cube_size: i32) -> Self {
        let mut map = HashMap::new();
        s.lines()
            .enumerate()
            .for_each(|(j,l)| l.chars()
                .enumerate()
                .for_each(|(i,c)| {
                    match c {
                        '.' => { map.insert((i as i32 + 1, j as i32 + 1), Tile::Open); },
                        '#' => { map.insert((i as i32 + 1, j as i32 + 1), Tile::Close); },
                        ' ' => {},
                        _ => panic!()
                    };
                }));
        let x = map.iter()
            .filter(|((_,y),tile)| *y == 1 && **tile == Tile::Open)
            .map(|((x,_),_)| *x)
            .min()
            .unwrap();
        let mut b = Board {
            map,
            x,
            y: 1,
            facing: Facing::Right,
            correspondence: HashMap::new(),
            cube_size
        };
        b.correspondence = get_face_vertices_correspondence(&assign_vertices_to_net(&b.get_net()));
        b
    }

    fn restart(&mut self) {
        self.y = 1;
        self.facing = Facing::Right;
        self.x = self.map.iter()
            .filter(|((_,y),tile)| *y == 1 && **tile == Tile::Open)
            .map(|((x,_),_)| *x)
            .min()
            .unwrap();
    }

    fn get_net(&self) -> HashSet<(i32,i32)> {
        let mut net = HashSet::new();
        for x in 0..4 {
            for y in 0..4 {
                if self.map.keys().contains(&(x*self.cube_size + 1,y*self.cube_size+1)) {
                    net.insert((x,y));
                }
            }
        }
        net
    }

    fn wrapped(&self, dx: i32, dy: i32) -> (i32,i32,Facing) {
        match (dx,dy) {
            (1,0) => {
                let new_x = self.map.keys()
                    .filter(|(_,v)| *v == self.y)
                    .map(|(u,_)| *u)
                    .min()
                    .unwrap();
                (new_x,self.y,self.facing)
            }
            (-1,0) => {
                let new_x = self.map.keys()
                    .filter(|(_,v)| *v == self.y)
                    .map(|(u,_)| *u)
                    .max()
                    .unwrap();
                (new_x,self.y,self.facing)
            }
            (0,1) => {
                let new_y = self.map.keys()
                    .filter(|(u,_)| *u == self.x)
                    .map(|(_,v)| *v)
                    .min()
                    .unwrap();
                (self.x,new_y,self.facing)
            }
            (0,-1) => {
                let new_y = self.map.keys()
                    .filter(|(u,_)| *u == self.x)
                    .map(|(_,v)| *v)
                    .max()
                    .unwrap();
                (self.x,new_y,self.facing)
            }
            _ => panic!()
        }
    }

    fn wrapped_cube(&self, dx: i32, dy: i32) -> (i32,i32,Facing){
        //identify current face
        let (face_x,face_y) = ((self.x-1) / self.cube_size, (self.y-1) / self.cube_size);
        //identity side
        let (side,mut side_pos) = match (dx,dy) {
            (1,0) => (Side::Right, (self.y - 1) % self.cube_size),
            (-1,0) => (Side::Left, (self.y - 1) % self.cube_size),
            (0,1) => (Side::Down, (self.x - 1) % self.cube_size),
            (0,-1) => (Side::Up, (self.x - 1) % self.cube_size),
            _ => panic!()
        };
        let curr_face_side = FaceSide { face: (face_x,face_y), side };
        let (new_face_side, orientation) = self.correspondence[&curr_face_side];
        let (x_top_left,y_top_left) = (new_face_side.face.0 * self.cube_size + 1,new_face_side.face.1 * self.cube_size + 1);
        if orientation == Direction::Reverse {
            side_pos = self.cube_size - 1 - side_pos;
        }
        let (dx_new,dy_new,new_facing) = match new_face_side.side {
            Side::Up => (side_pos,0, Facing::Down),
            Side::Right => (self.cube_size-1,side_pos, Facing::Left),
            Side::Down => (side_pos, self.cube_size-1, Facing::Up),
            Side::Left => (0,side_pos, Facing::Right),
        };
        (x_top_left+dx_new,y_top_left+dy_new,new_facing)
    }

    fn apply_instruction(&mut self, instr: &Instruction, wrapping_cube: bool) {
        match instr {
            Instruction::Turn(rot) => {self.facing = self.facing.rotate(rot);}
            Instruction::Move(mv) => {
                for _ in 0..*mv {
                    let (dx,dy) = self.facing.get_movement();
                    if let Some(tile) = self.map.get(&(self.x+dx,self.y+dy)) {
                        match tile {
                            Tile::Open => {
                                self.x += dx;
                                self.y += dy;
                            }
                            Tile::Close => {
                                break;
                            }
                        }
                    }
                    else {
                        let (new_x,new_y,new_facing) = match wrapping_cube {
                            true => self.wrapped_cube(dx,dy),
                            false => self.wrapped(dx,dy)
                        };
                        match self.map[&(new_x,new_y)] {
                            Tile::Open => {
                                self.x = new_x;
                                self.y = new_y;
                                self.facing = new_facing;
                            }
                            Tile::Close => {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_password(&self) -> i32 {
        self.y * 1000 + self.x * 4 + (self.facing as i32)
    }
}

impl Instruction {
    fn parse_line(s: &str) -> Vec<Self> {
        let mut s_commas = String::new();
        s.chars()
            .for_each(|c| {
                if c.is_alphabetic() {
                    s_commas.push(',');
                    s_commas.push(c);
                    s_commas.push(',');
                }
                else {
                    s_commas.push(c);
                }
            });
        s_commas.split(',')
            .map(|part| if let Ok(mv) = part.parse::<i32>() {
                Instruction::Move(mv)
            } else {
                match part {
                    "L" => Instruction::Turn(Rotation::Counterclockwise),
                    "R" => Instruction::Turn(Rotation::Clockwise),
                    _ => panic!()
                }
            })
            .collect()
    }
}

fn assign_vertices_to_net(net: &HashSet<(i32,i32)>) -> HashMap<(i32,i32),[[char;2];2]>{
    let dirs = vec![CubeRoll::Up,CubeRoll::Down,CubeRoll::Left,CubeRoll::Right];
    let mut visited_faces = HashMap::new();
    let mut to_visit = vec![(*net.iter().next().unwrap(),Cube::new())];
    while !to_visit.is_empty() {
        let ((x,y),cube) = to_visit.pop().unwrap();
        visited_faces.insert((x,y),cube.vertices[0]);
        for roll in dirs.iter() {
            let (dx,dy) = roll.get_disp();
            let n_face = (x+dx,y+dy);
            if net.contains(&n_face) && !visited_faces.contains_key(&n_face) {
                to_visit.push((n_face,cube.roll(roll)));
            }
        }
    }
    visited_faces
}

fn get_face_vertices_correspondence(net_vertices: &HashMap<(i32,i32),[[char;2];2]>) -> HashMap<FaceSide,(FaceSide,Direction)> {
    let mut result = HashMap::new();
    let sides: Vec<Side> = vec![Side::Up,Side::Down,Side::Right,Side::Left];
    let mut buffer = HashSet::new();
    for (x,y) in net_vertices.keys() {
        for side in sides.iter() {
            let (dx,dy) = side.get_disp();
            if !net_vertices.keys().contains(&(x+dx,y+dy)) {
                let vertices = net_vertices[&(*x,*y)];
                let (c1,c2) = match side {
                    Side::Up => (vertices[0][0], vertices[0][1]),
                    Side::Right => (vertices[0][1], vertices[1][1]),
                    Side::Down => (vertices[1][0], vertices[1][1]),
                    Side::Left => (vertices[0][0], vertices[1][0]),
                };
                buffer.insert((FaceSide { face: (*x,*y), side: *side },c1,c2));
            }

        }
    }
    for (face1,sa1,sa2) in buffer.iter() {
        for (face2,sb1,sb2) in buffer.iter() {
            if face1 != face2 {
                if sa1 == sb1 && sa2 == sb2 {
                    result.insert(*face1,(*face2,Direction::Normal));
                }
                else if sa1 == sb2 && sa2 == sb1 {
                    result.insert(*face1,(*face2,Direction::Reverse));
                }
            }
        }
    }
    result
}

pub fn day22() {
    let test = false;
    let (path,cube_size) = match test {
        true => ("./inputs/input_day22t.txt",4),
        false => ("./inputs/input_day22.txt",50),
    };
    let mut file = File::open(path).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    
    let mut data_it = data.split("\n\n");
    let mut board = Board::parse(data_it.next().unwrap(),cube_size);
    let instructions = Instruction::parse_line(data_it.next().unwrap());

    for inst in instructions.iter() {
        board.apply_instruction(inst,false);
    }

    println!("Solution 1 : {}",board.get_password());

    board.restart();

    for inst in instructions.iter() {
        board.apply_instruction(inst,true);
    }

    println!("Solution 2 : {}",board.get_password());

}