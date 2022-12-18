use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use itertools::Itertools;
use pathfinding::prelude::connected_components;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinates(i32,i32,i32);

impl Coordinates {
    fn neighbors(&self) -> Vec<Coordinates> {
        vec![(1,0,0),(0,1,0),(0,0,1),(-1,0,0),(0,-1,0),(0,0,-1)].iter()
            .map(|(x,y,z)| Coordinates(self.0 + x,self.1 + y, self.2 + z))
            .collect()
    }

    fn inside(&self, min_coord: Coordinates, max_coord: Coordinates) -> bool {
        self.0 >= min_coord.0 && self.0 <= max_coord.0 &&
            self.1 >= min_coord.1 && self.1 <= max_coord.1 &&
            self.2 >= min_coord.2 && self.2 <= max_coord.2
    }
}
pub fn day18() {
    let mut file = File::open("./inputs/input_day18.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let lava_cubes: HashSet<Coordinates> = data.lines()
        .map(|line| {
            let t = line.split(',')
                .map(|nb| nb.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            Coordinates(t[0],t[1],t[2])})
        .collect();

    let mut nb_faces = 0;
    for cube in &lava_cubes {
        let mut nb_faces_cube = 6;
        for neighbor in cube.neighbors().iter() {
            if lava_cubes.contains(neighbor) {
                nb_faces_cube -= 1;
            }
        }
        nb_faces += nb_faces_cube;
    }
    println!("Solution 1 : {}",nb_faces);

    let min_coordinate = lava_cubes.iter().cloned()
        .reduce(|accum,item| Coordinates(accum.0.min(item.0),accum.1.min(item.1),accum.2.min(item.2)))
        .unwrap();
    let max_coordinate = lava_cubes.iter().cloned()
        .reduce(|accum,item| Coordinates(accum.0.max(item.0),accum.1.max(item.1),accum.2.max(item.2)))
        .unwrap();

    let min_coordinate = Coordinates(min_coordinate.0-1,min_coordinate.1-1,min_coordinate.2-1);
    let max_coordinate = Coordinates(max_coordinate.0+1,max_coordinate.1+1,max_coordinate.2+1);

    let air_cubes = (min_coordinate.0..=max_coordinate.0)
        .cartesian_product(min_coordinate.1..=max_coordinate.1)
        .cartesian_product(min_coordinate.2..=max_coordinate.2)
        .map(|((x,y),z)| Coordinates(x,y,z))
        .collect::<HashSet<Coordinates>>()
        .difference(&lava_cubes)
        .copied()
        .collect::<Vec<Coordinates>>();

    let neighbors_closure = |cube: &Coordinates| cube.neighbors().iter()
        .filter(|neighbor| neighbor.inside(min_coordinate,max_coordinate) && !lava_cubes.contains(neighbor))
        .copied()
        .collect::<Vec<Coordinates>>();

    let components = connected_components(&air_cubes,neighbors_closure);

    let outside_component = components.iter()
        .find(|component| component.contains(&min_coordinate))
        .unwrap();

    let mut nb_faces = 0;
    for cube in &lava_cubes {
        let mut nb_faces_cube = 0;
        for neighbor in cube.neighbors().iter() {
            if outside_component.contains(neighbor) {
                nb_faces_cube += 1;
            }
        }
        nb_faces += nb_faces_cube;
    }
    println!("Solution 2 : {}",nb_faces);
}