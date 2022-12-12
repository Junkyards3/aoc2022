use std::fs::File;
use std::io::Read;
use ndarray::{Array2, Axis, Dimension};
use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32,i32);

impl Pos {
    fn neighbors(&self, grid: &Array2<char>) -> Vec<(Pos, usize)> {
        let &Pos(x,y) = self;
        let (height,width) = grid.raw_dim().into_pattern();
        let own_elevation = convert_to_elevation(grid[[x as usize,y as usize]]);
        vec![(x+1,y),(x-1,y),(x,y+1),(x,y-1)]
            .into_iter()
            .filter(|(u,v)| *u >= 0 && *v >= 0 && *u < height as i32 && *v < width as i32 &&
                convert_to_elevation(grid[[*u as usize,*v as usize]]) <= own_elevation + 1)
            .map(|(u,v)| (Pos(u,v),1))
            .collect()


    }
}
fn convert_to_elevation(c : char) -> u32 {
    match c {
        'S' => 0,
        'E' => 25,
        c => c as u32 - 'a' as u32
    }
}
pub fn day12() {
    let mut file = File::open("./inputs/input_day12.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let height = data.split('\n').count();
    let width = data.split('\n').next().unwrap().len();

    let mut array: Array2<char> = Array2::default((height,width));
    data.split('\n')
        .zip(array.axis_iter_mut(Axis(0)))
        .for_each(|(u,mut row)| u.chars().zip(row.iter_mut())
            .for_each(|(c,elevation)| *elevation = c));

    let start = array.indexed_iter().find(|((_x,_y),c)| **c == 'S').unwrap();
    let start = Pos(start.0.0 as i32, start.0.1 as i32);

    let end = array.indexed_iter().find(|((_x,_y),c)| **c == 'E').unwrap();
    let end: Pos = Pos(end.0.0 as i32, end.0.1 as i32);

    let path = dijkstra(&start, |p| p.neighbors(&array), |p| *p == end).unwrap();
    println!("Solution 1 : {:?}",path.0.len() - 1);

    let min_length = array.indexed_iter()
        .filter(|((_x,_y),c)| convert_to_elevation(**c) == 0)
        .filter_map(|((x,y),_)| {
            dijkstra(&Pos(x as i32,y as i32), |p| p.neighbors(&array), |p| *p == end)
                .map(|v| v.0.len())
        })
        .min().unwrap() - 1;


    println!("Solution 2 : {:?}",min_length);

}