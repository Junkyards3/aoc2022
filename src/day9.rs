use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Default)]
struct Head {
    pos_x: i32,
    pos_y: i32,
}

#[derive(Debug, Default, Clone)]
struct Tail {
    pos_x: i32,
    pos_y: i32,
}

fn d_inf(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    max((p1.0 - p2.0).abs(),(p1.1 - p2.1).abs())
}
impl Tail {
    fn closest_between(&self, p1: (i32,i32), p2: (i32,i32)) -> (i32,i32) {
        let d1 = d_inf((self.pos_x,self.pos_y),p1);
        let d2 = d_inf((self.pos_x,self.pos_y),p2);
        if d1 < d2 {
            p1
        }
        else {
            p2
        }
    }
    fn move_from_followed_move(&mut self, movem: (i32,i32), followed_pos: (i32,i32)) {
        let (dx,dy) = movem;
        let (xn,yn) = followed_pos;
        let (x0,y0) = (xn - dx,yn - dy);
        if dx == 0 || dy == 0 {
            if (xn - self.pos_x).abs() == 2 || (yn - self.pos_y).abs() == 2{
                (self.pos_x, self.pos_y) = (x0,y0)
            }
        }
        else{
            let anti_diag = (x0 - dx, y0 - dy);
            if d_inf((self.pos_x,self.pos_y),followed_pos) <= 1 {

            }
            else if (self.pos_x, self.pos_y) == anti_diag {
                (self.pos_x, self.pos_y) = (x0,y0)
            }
            else {
                (self.pos_x, self.pos_y) = self.closest_between((x0, y0 + dy),(x0 + dx, y0));
            }
        }
    }
}
struct SystemMultipleTails {
    head: Head,
    tails: Vec<Tail>,
    visited_last_tail_pos: HashSet<(i32,i32)>
}

impl SystemMultipleTails{
    fn new(nb_tails: usize) -> Self {
        SystemMultipleTails {
            head: Default::default(),
            tails: vec![Default::default();nb_tails],
            visited_last_tail_pos: HashSet::new(),
        }
    }

    fn move_head(&mut self, move_head: (i32,i32)) {
        self.head.pos_x += move_head.0;
        self.head.pos_y += move_head.1;
        let mut curr_pos = (self.head.pos_x, self.head.pos_y);
        let mut movet = move_head;
        for t in self.tails.iter_mut(){
            let (xb,yb) = (t.pos_x,t.pos_y);
            t.move_from_followed_move(movet,curr_pos);
            movet = (t.pos_x - xb, t.pos_y - yb);
            curr_pos = (t.pos_x,t.pos_y);
        }
        let last_tail = self.tails.last().unwrap();
        self.visited_last_tail_pos.insert((last_tail.pos_x,last_tail.pos_y));
    }
}
pub fn day9() {
    let mut file = File::open("./inputs/input_day9.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let inst: Vec<(i32,i32)> = data.split('\n')
        .map(|s| {
            let mut it = s.split_whitespace();
            let displacement = match it.next().unwrap() {
                "R" => (1,0),
                "U" => (0,1),
                "L" => (-1,0),
                "D" => (0,-1),
                _ => panic!()
            };
            let n: usize = it.next().unwrap().parse().unwrap();
            vec![displacement; n]
        })
        .reduce(|accum, item| [accum,item].concat())
        .unwrap();
    let mut system1 = SystemMultipleTails::new(1);
    let mut system10 = SystemMultipleTails::new(9);
    inst.iter()
        .for_each(|movet| {
            system1.move_head(*movet);
            system10.move_head(*movet);
        });
    println!("Solution 1 : {:?}", system1.visited_last_tail_pos.len());
    println!("Solution 2 : {:?}", system10.visited_last_tail_pos.len());
}