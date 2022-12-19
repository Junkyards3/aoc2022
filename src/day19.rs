use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::day19::Resource::{Geode, Obsidian};

#[derive(Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    robots_cost: [[u32;4];4],
    table: HashMap<(u32,[u32;4],[u32;4]),u32>,
    curr_max: u32,
    time : u32,
}

impl Blueprint {
    fn parse(line: &str, time : u32) -> Blueprint {
        let mut robots_cost: [[u32;4];4] = [[0;4];4];
        let colon_index = line.find(':').unwrap();
        line[colon_index+1..].split('.')
            .take(4)
            .enumerate()
            .for_each(|(i,s)| {
                let nbs = s.split_whitespace()
                    .filter_map(|word| word.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
                robots_cost[i][0] = nbs[0];
                match i {
                    0 | 1 => {}
                    2 => {robots_cost[i][1] = nbs[1]},
                    3 => {robots_cost[i][2] = nbs[1]},
                    _ => panic!()
                }
            });
        Blueprint { robots_cost, table: HashMap::new(), curr_max: 0, time }

    }

    fn can_make_robot(&self, resources: &[u32], robot_index: usize) -> Option<[u32;4]> {
        let mut resources_after = [0;4];
        for i in 0..4 {
            if resources[i] < self.robots_cost[robot_index][i] {
                return None
            }
            resources_after[i] = resources[i] - self.robots_cost[robot_index][i];
        }
        Some(resources_after)
    }

    fn get_max_geodes(&mut self) -> u32 {
        self.get_max_geodes_table(0,[0,0,0,0],[1,0,0,0])
    }

    fn get_max_geodes_table(&mut self, time: u32, resources: [u32;4], robots: [u32;4]) -> u32 {
        let key = (time,resources,robots);
        if let Some(value) = self.table.get(&key){
            *value
        }
        else {
            if let Some(v) = self.get_max_geodes_helper(time, resources, robots){
                self.table.insert(key,v);
                return v
            }
            0
        }
    }

    fn get_max_geodes_helper(&mut self, time: u32, resources: [u32;4], robots: [u32;4]) -> Option<u32> {
        if time == self.time {
            return Some(resources[3])
        }
        let mut new_resources = [0_u32;4];
        for j in 0..4 {
            new_resources[j] = resources[j] + robots[j];
        }
        let mut scores = Vec::new();
        if let Some(score) = self.heuristic_and_compute(time+1,new_resources,robots) {
            scores.push(score);
        }
        for robot_index in 0..4 {
            if let Some(mut new_resources_rob) = self.can_make_robot(&resources,robot_index) {
                let mut new_robots = robots;
                new_robots[robot_index] += 1;
                for j in 0..4 {
                    new_resources_rob[j] += robots[j];
                }
                if let Some(score) = self.heuristic_and_compute(time+1,new_resources_rob,new_robots) {
                    scores.push(score);
                    self.curr_max = self.curr_max.max(score);
                }
            }
        }
        scores.iter().max().copied()
    }

    fn heuristic_and_compute(&mut self, time: u32, resources: [u32;4], robots: [u32;4]) -> Option<u32> {
        //heuristic : consider you can make one obsidian robot per turn, how many geode robots will you be able to make (disregarding ore)
        let n = self.time - time;
        let obsidian_quantity = resources[Obsidian as usize] + robots[Obsidian as usize] * n + (n*(n+1))/2;
        let nb_of_turns_making_geode_robot = obsidian_quantity / self.robots_cost[Geode as usize][Obsidian as usize];
        let m = min(n,nb_of_turns_making_geode_robot);
        let heuristic = resources[Geode as usize] + robots[Geode as usize] * n + (m*(m+1))/2;
        if heuristic <= self.curr_max {
            None
        }
        else {
            Some(self.get_max_geodes_table(time,resources,robots))
        }
    }
}

pub fn day19() {
    let mut file = File::open("./inputs/input_day19.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let sol1: u32 = data.lines()
        .enumerate()
        .map(|(i,l)| Blueprint::parse(l,24).get_max_geodes() * (i as u32 + 1))
        .sum();

    println!("Solution 1 : {sol1}");

    let sol2: u32 = data.lines()
        .take(3)
        .map(|l| Blueprint::parse(l,32).get_max_geodes())
        .product();

    println!("Solution 2 : {:?}",sol2);
}