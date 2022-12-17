use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use bubblemath::linear_recurrence::berlekamp_massey;

#[derive(Debug)]
enum Push {
    Left,
    Right,
}

//use the bottom-left space as origin (even if empty as in case of rock type 2)
#[derive(Debug, Clone)]
struct Rock {
    pos_x: i32,
    pos_y: i32,
    relative_pos_rocks: Vec<(i32,i32)>
}

impl Rock {
    fn make_rock1() -> Self {
        Rock { pos_x: 0 , pos_y: 0, relative_pos_rocks: vec![(0,0),(1,0),(2,0),(3,0)],
        }
    }
    fn make_rock2() -> Self {
        Rock { pos_x: 0 , pos_y: 0, relative_pos_rocks: vec![(0,1),(1,0),(1,1),(1,2),(2,1)],
        }
    }

    fn make_rock3() -> Self {
        Rock { pos_x: 0 , pos_y: 0, relative_pos_rocks: vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        }
    }
    fn make_rock4() -> Self {
        Rock { pos_x: 0 , pos_y: 0, relative_pos_rocks: vec![(0,0),(0,1),(0,2),(0,3)],
        }
    }
    fn make_rock5() -> Self {
        Rock { pos_x: 0 , pos_y: 0, relative_pos_rocks: vec![(0,0),(1,0),(0,1),(1,1)],
        }
    }

    fn set_pos(&mut self, max_rock_height: i32) {
        self.pos_x = 3;
        self.pos_y = max_rock_height + 4;
    }

    fn collide_with_rocks(&self, rocks: &HashSet<(i32,i32)>) -> bool {
        let set_rocks = self.relative_pos_rocks.iter()
            .map(|(x,y)| (x + self.pos_x,y + self.pos_y))
            .collect::<HashSet<(i32,i32)>>();
        //check left, right, bottom side, rocks
        set_rocks.iter().any(|p| p.0 <= 0 || p.0 >= 8 || p.1 <= 0 || rocks.contains(p))
    }

    //returns the new rocks if it has stopped
    fn has_stopped_after_complete_move(&mut self, rocks: &HashSet<(i32, i32)>, push: Push) -> Option<impl Iterator<Item=(i32,i32)> + '_>  {
        //horizontal
        let disp_x = match push {
            Push::Left => -1,
            Push::Right => 1
        };
        self.pos_x += disp_x;
        if self.collide_with_rocks(rocks) {
            self.pos_x -= disp_x;
        }
        //vertical
        self.pos_y -= 1;
        if self.collide_with_rocks(rocks) {
            self.pos_y += 1;
            let new_rocks = self.relative_pos_rocks.iter()
                .map(|(x,y)| (x + self.pos_x,y + self.pos_y));
            return Some(new_rocks);
        }
        None
    }
}

pub fn day17() {
    let mut file = File::open("./inputs/input_day17.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut pushes = data.chars()
        .map(|c| match c {
            '>' => Push::Right,
            '<' => Push::Left,
            _ => panic!()
        })
        .cycle();

    let mut rocks: HashSet<(i32,i32)> = HashSet::new();
    let rock_types = vec![Rock::make_rock1(),Rock::make_rock2(),Rock::make_rock3(),Rock::make_rock4(),Rock::make_rock5()];
    let rock_iterator = rock_types
        .iter()
        .cycle()
        .take(5000);
    let mut curr_height = 0;
    let mut heights = vec![curr_height as u64];
    // compute values of the heights
    for rock in rock_iterator {
        let mut rock_copy = rock.clone();
        rock_copy.set_pos(curr_height);
        loop {
            if let Some(new_rocks) = rock_copy.has_stopped_after_complete_move(&rocks,pushes.next().unwrap()) {
                new_rocks
                    .for_each(|(x,y)| {
                        rocks.insert((x, y));
                        curr_height = curr_height.max(y);
                    });
                break;
            }
        }
        heights.push(curr_height as u64);
    }
    //use theses values to compute a linear recurrence relation
    let heights_diff = heights.windows(2)
        .map(|x| x[1] - x[0])
        .collect::<Vec<u64>>();
    let p = 2147480009;
    let recurrence_coeffs = berlekamp_massey(&heights_diff, p).iter()
        .rev()
        .enumerate()
        .filter(|(_i,x)| **x != 0)
        .map(|(i,&x)| {
            if x > p/2 {
                (i+1,x as i64 - p as i64)
            }
            else {
                (i+1,x as i64)
            }
        })
        .collect::<Vec<(usize,i64)>>();


    // use the relation to compute the desired height
    let size_rel = *recurrence_coeffs.iter().map(|(i,_v)| i).max().unwrap();
    let desired_heights = [(1,2022),(2,1000000000000)];

    for (index,desired_height) in desired_heights.iter() {
        let coeff_to_get_inside_heights = if *desired_height > heights.len() {((desired_height - heights.len())/size_rel) + 1} else {0};
        let index_inside_heights = desired_height - size_rel * coeff_to_get_inside_heights;
        let size_every_period = heights_diff.iter().rev().take(size_rel).sum::<u64>() as usize;

        println!("Solution {index} : {}",heights[index_inside_heights] as usize + coeff_to_get_inside_heights * size_every_period)
    }


}