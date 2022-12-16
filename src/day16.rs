use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Time(u32);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct ValveId(String);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct BitSetOperable(usize);

fn get_rank(valve_id: ValveId, operable_ids: &[ValveId]) -> Option<usize> {
    operable_ids.iter()
        .position(|id| *id == valve_id)
}
impl BitSetOperable {
    fn contains(&self, valve_id: ValveId, operable_ids: &[ValveId]) -> bool {
        get_rank(valve_id,operable_ids).map(|r| (1 << r) & self.0 != 0).unwrap_or(false)
    }

    fn remove(&self, valve_id: ValveId, operable_ids: &[ValveId]) -> BitSetOperable {
        BitSetOperable(self.0 - (1 << get_rank(valve_id,operable_ids).unwrap()))
    }
}
#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbors: Vec<ValveId>,
}

fn fill_dynamic(array : &mut HashMap<(Time, ValveId, BitSetOperable),u32>, operable_ids: &[ValveId], valves: &HashMap<ValveId,Valve>) {
    for time in 1..30 {
        for pos in valves.keys() {
            for x in 0..2_usize.pow(operable_ids.len() as u32) {
                let b = BitSetOperable(x);
                let key = (Time(time),pos.clone(),b);
                let mut o = *array.get(&key).unwrap_or(&0);

                if b.contains(pos.clone(),operable_ids) {
                    let key_b = (Time(time-1),pos.clone(),b.remove(pos.clone(),operable_ids));
                    o = o.max(array.get(&key_b).unwrap_or(&0) + valves[pos].flow_rate * time);
                }

                for pos_neighbor in valves[pos].neighbors.iter() {
                    let key_b = (Time(time-1),pos_neighbor.clone(),b);
                    o = o.max(*array.get(&key_b).unwrap_or(&0))
                }
                array.insert(key,o);
            }
        }
    }
}
fn parse(line: &str) -> (Valve,ValveId) {
    let mut it = line.split(';');
    let mut first_part = it.next().unwrap().split_whitespace();
    let valve_id = first_part.nth(1).unwrap();
    let flow_rate = first_part.last().unwrap().split('=').last().unwrap().parse::<u32>().unwrap();
    let neighbors = it.next().unwrap().split_whitespace().skip(4)
        .map(|s| ValveId(s.chars().filter(|&c| c!=',').collect::<String>())).collect();
    (Valve { flow_rate, neighbors }, ValveId(valve_id.to_owned()))
}

pub fn day16() {
    let mut file = File::open("./inputs/input_day16.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut valves = HashMap::new();
    data.lines()
        .for_each(|l| {
            let (valve,valve_id) = parse(l);
            valves.insert(valve_id,valve);
        });
    let operable_valves = valves.iter()
        .filter(|(_id,valve)| valve.flow_rate != 0)
        .map(|(id,_valve)| id.clone())
        .collect::<Vec<ValveId>>();

    let nb_op = operable_valves.len();
    let bitset_all = BitSetOperable((1 << nb_op) - 1);
    let mut array = HashMap::new();
    fill_dynamic(&mut array,&operable_valves,&valves);

    println!("Solution 1 : {:?}",array.get(&(Time(29), ValveId(String::from("AA")), bitset_all)).unwrap());

    let mut best = 0;
    for x in 0..(1 << nb_op)/2 {
        let b1 = BitSetOperable(x);
        let b2 = BitSetOperable((1 << nb_op) - 1 - x);
        best = best.max(array[&(Time(25), ValveId(String::from("AA")), b1)] + array[&(Time(25), ValveId(String::from("AA")), b2)])
    }

    println!("Solution 2 : {:?}",best);
}