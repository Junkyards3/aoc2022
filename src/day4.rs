use std::fs::File;
use std::io::Read;
use std::str::FromStr;

struct ElvesPair {
    start_first: u32,
    end_first: u32,
    start_second: u32,
    end_second: u32
}

impl ElvesPair {
    fn contains_other(&self) -> bool {
        (self.start_first >= self.start_second && self.end_first <= self.end_second) ||
            (self.start_first <= self.start_second && self.end_first >= self.end_second)
    }

    fn overlap(&self) -> bool {
        self.end_first >= self.start_second && self.start_first <= self.end_second
    }
}

impl FromStr for ElvesPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',')
            .map(|sections| {
                sections.split('-')
                    .map(|section| u32::from_str(section).unwrap())
                    .collect::<Vec<u32>>()
            });
        let sec1 = it.next().unwrap();
        let sec2 = it.next().unwrap();
        Ok(ElvesPair {
            start_first: sec1[0],
            end_first: sec1[1],
            start_second: sec2[0],
            end_second: sec2[1],
        })
    }
}

pub fn day4() {
    let mut file = File::open("./inputs/input_day4.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let nb_contains_other = data.split('\n')
        .map(|str| ElvesPair::from_str(str).unwrap())
        .filter(|pair| pair.contains_other())
        .count();

    let nb_overlap = data.split('\n')
        .map(|str| ElvesPair::from_str(str).unwrap())
        .filter(|pair| pair.overlap())
        .count();

    println!("Solution 1 : {:?}", nb_contains_other);
    println!("Solution 2 : {:?}", nb_overlap);
}