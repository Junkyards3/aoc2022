use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

struct CircularSequence {
    initial_order: Vec<(i64,u32)>,
    left_neighbors: HashMap<(i64,u32),(i64,u32)>,
    right_neighbors: HashMap<(i64,u32),(i64,u32)>,
    number_of_holes: i64,
}

impl CircularSequence {
    fn make_from_slice(v : &[i64]) -> Self {
        let mut left_neighbors = HashMap::new();
        let mut right_neighbors = HashMap::new();
        let mut nbs_occurrences: HashMap<i64,u32> = HashMap::new();
        let mut initial_order = Vec::new();

        for nbs in v.windows(2) {
            let key_nb0 = (nbs[0],*nbs_occurrences.get(&nbs[0]).unwrap_or(&0));
            nbs_occurrences.entry(nbs[0]).and_modify(|occ| *occ += 1).or_insert(1);
            let key_nb1 = (nbs[1],*nbs_occurrences.get(&nbs[1]).unwrap_or(&0));
            left_neighbors.insert(key_nb1,key_nb0);
            right_neighbors.insert(key_nb0,key_nb1);
            initial_order.push(key_nb0);
        }
        let first_nb = *v.iter().next().unwrap();
        let last_nb = *v.iter().last().unwrap();
        let key_first = (first_nb,0);
        let key_last = (last_nb, *nbs_occurrences.get(&last_nb).unwrap_or(&0));
        initial_order.push(key_last);
        left_neighbors.insert(key_first,key_last);
        right_neighbors.insert(key_last,key_first);
        CircularSequence { initial_order, left_neighbors, right_neighbors, number_of_holes: (v.len()-1) as i64 }
    }

    fn mix(&mut self) {
        for number in self.initial_order.iter() {
            let prev_left_neighbor = self.left_neighbors[number];
            let prev_right_neighbor = self.right_neighbors[number];
            let mut new_right_neighbor = prev_right_neighbor;
            let mut new_left_neighbor = prev_left_neighbor;
            let displacement = self.get_modulo(number.0);
            match displacement.cmp(&0) {
                Ordering::Less => {
                    new_left_neighbor = prev_left_neighbor;
                    for _ in 0..(-displacement) {
                        new_left_neighbor = self.left_neighbors[&new_left_neighbor];
                    }
                    new_right_neighbor = self.right_neighbors[&new_left_neighbor];
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    new_right_neighbor = prev_right_neighbor;
                    for _ in 0..displacement {
                        new_right_neighbor = self.right_neighbors[&new_right_neighbor];
                    }
                    new_left_neighbor = self.left_neighbors[&new_right_neighbor];
                }
            }
            if displacement != 0 {
                //change neighbors of current number
                self.right_neighbors.insert(*number, new_right_neighbor);
                self.left_neighbors.insert(*number, new_left_neighbor);
                //change left neighbor of right neighbor
                self.left_neighbors.insert(new_right_neighbor, *number);
                //change right neighbor of last neighbor
                self.right_neighbors.insert(new_left_neighbor,*number);
                //prev left neighbor
                self.right_neighbors.insert(prev_left_neighbor,prev_right_neighbor);
                //prev right neighbor
                self.left_neighbors.insert(prev_right_neighbor,prev_left_neighbor);
            }
        }
    }

    fn get_modulo(&self, nb: i64) -> i64 {
        let mut modulo = ((nb % self.number_of_holes) + self.number_of_holes) % self.number_of_holes;
        if modulo > self.number_of_holes / 2 {
            modulo -= self.number_of_holes
        }
        modulo
    }
    fn get_vec_starting_at_zero(&self) -> Vec<i64> {
        let mut starting_at_zero = vec![0];
        let mut curr_value = self.right_neighbors[&(0,0)];
        while curr_value != (0,0) {
            starting_at_zero.push(curr_value.0);
            curr_value = self.right_neighbors[&curr_value];
        }
        starting_at_zero
    }

    fn sum_grove_coordinates(&self) -> i64 {
        let u = self.get_vec_starting_at_zero();
        u[1000 % u.len()] + u[2000 % u.len()] + u[3000 % u.len()]
    }
}
pub fn day20() {
    let mut file = File::open("./inputs/input_day20.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let nbs = data.lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut circ_seq = CircularSequence::make_from_slice(&nbs);
    circ_seq.mix();
    println!("Solution 1 : {:?}", circ_seq.sum_grove_coordinates());

    let decrypt_key = 811589153;
    let nbs = data.lines()
        .map(|s| s.parse::<i64>().unwrap() * decrypt_key)
        .collect::<Vec<i64>>();
    let mut circ_seq = CircularSequence::make_from_slice(&nbs);
    for _ in 0..10 {
        circ_seq.mix();
    }
    println!("Solution 2 : {:?}", circ_seq.sum_grove_coordinates());

}