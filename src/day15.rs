use std::fs::File;
use std::io::Read;
use interval::interval_set::ToIntervalSet;
use interval::IntervalSet;
use gcollections::ops::*;

fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

#[derive(Debug)]
struct Sensor {
    pos_x: i32,
    pos_y: i32,
    range_no_beacon: i32,
}

impl Sensor {
    fn get_no_beacon_row(&self, y: i32) -> IntervalSet<i32> {
        let displacement = self.range_no_beacon - (self.pos_y - y).abs();
        if displacement > 0 {
            vec![(self.pos_x - displacement,self.pos_x + displacement)].to_interval_set()
        }
        else {
            IntervalSet::empty()
        }
    }
}

fn parse(line: &str) -> (Sensor,i32) {
    let nbs = line.split(&[':',','])
        .map(|s| s.chars()
            .filter(|c| c.is_numeric() || *c == '-')
            .collect::<String>().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let range = distance((nbs[0],nbs[1]),(nbs[2],nbs[3]));
    (Sensor {
        pos_x: nbs[0],
        pos_y: nbs[1],
        range_no_beacon: range,
    },nbs[3])
}

pub fn day15() {
    let mut file = File::open("./inputs/input_day15t.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let row = 10;
    let interval_size = data.lines()
        .map(|l| parse(l).0.get_no_beacon_row(row))
        .reduce(|accum, item| accum.union(&item))
        .unwrap()
        .size();
    let sol1 = interval_size - data.lines()
        .map(parse)
        .filter(|(_sensor,row_beacon)| *row_beacon == row)
        .count() as u32;
    println!("Solution 1 : {:?}", sol1)

}