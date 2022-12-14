use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;

fn get_line(p1: (i32,i32), p2: (i32,i32)) -> Vec<(i32,i32)> {
    if p1.0 == p2.0 {
        let mut vy = vec![p1.1,p2.1];
        vy.sort_unstable();
        (vy[0]..=vy[1]).map(|y| (p1.0,y)).collect()
    }
    else {
        let mut vx = vec![p1.0,p2.0];
        vx.sort_unstable();
        (vx[0]..=vx[1]).map(|x| (x,p1.1)).collect()
    }
}

fn put_sand_and_check_abyss(cave: &mut HashSet<(i32,i32)>, abyss_height: i32) -> bool {
    let (mut x, mut y) = (500,0);
    loop {
        if y >= abyss_height {
            return true
        }
        else if !cave.contains(&(x,y+1)) {
            y += 1;
        }
        else if !cave.contains(&(x-1,y+1)) {
            x -= 1;
            y += 1;
        }
        else if !cave.contains(&(x+1,y+1)) {
            x += 1;
            y += 1;
        }
        else {
            cave.insert((x,y));
            return false
        }
    }
}

fn put_sand_floor_and_check_source(cave: &mut HashSet<(i32,i32)>, height_floor: i32) -> bool {
    let (mut x, mut y) = (500,0);
    loop {
        if y == height_floor - 1 {
            cave.insert((x,y));
            return (x,y) == (500,0);
        }
        else if !cave.contains(&(x,y+1)) {
            y += 1;
        }
        else if !cave.contains(&(x-1,y+1)) {
            x -= 1;
            y += 1;
        }
        else if !cave.contains(&(x+1,y+1)) {
            x += 1;
            y += 1;
        }
        else {
            cave.insert((x,y));
            return (x,y) == (500,0);
        }
    }
}
pub fn day14() {
    let mut file = File::open("./inputs/input_day14.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut cave: HashSet<(i32,i32)> = HashSet::new();
    data.lines()
        .for_each(|line| line.split(" -> ")
            .map(|pair| {
                let mut pair_nb = pair.split(',');
                let width = pair_nb.next().unwrap().parse::<i32>().unwrap();
                let height = pair_nb.next().unwrap().parse::<i32>().unwrap();
                (width,height)
            })
            .tuple_windows::<(_,_)>()
            .for_each(|(t1,t2)| {
                get_line(t1,t2).iter()
                    .for_each(|(x,y)| { cave.insert((*x, *y)); });
            }));
    let height_abyss = cave.iter().map(|t| t.1).max().unwrap();
    let mut cave2 = cave.clone();

    let mut nb_sand = 0;
    while !put_sand_and_check_abyss(&mut cave,height_abyss) {
        nb_sand += 1
    }

    let mut nb_sand2 = 1;
    while !put_sand_floor_and_check_source(&mut cave2,height_abyss+2) {
        nb_sand2 += 1
    }
    println!("Solution 1 : {:?}",nb_sand);
    println!("Solution 2 : {:?}",nb_sand2);
}