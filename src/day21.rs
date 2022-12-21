use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::day21::Operation::{Add, Constant, Div, Mul, Root, Sub, Variable};


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct MonkeyId(String);

#[derive(Debug)]
enum Operation {
    Constant(i64),
    Variable,
    Add(MonkeyId,MonkeyId),
    Sub(MonkeyId,MonkeyId),
    Mul(MonkeyId,MonkeyId),
    Div(MonkeyId,MonkeyId),
    Root(MonkeyId,MonkeyId)
}

#[derive(Debug)]
struct Monkey {
    id: MonkeyId,
    op: Operation
}

impl Monkey {
    fn parse(s: &str, is_part2: bool) -> Self {
        let mut it = s.split(": ");
        let monkey_id = MonkeyId(it.next().unwrap().to_owned());
        let entire_op = it.next().unwrap().split_whitespace().collect::<Vec<&str>>();
        let op = if entire_op.len() == 1 {
            if is_part2 && monkey_id.0 == "humn" {
                Variable
            }
            else {
                Constant(entire_op[0].parse::<i64>().unwrap())
            }
        }
        else {
            let operand1 = MonkeyId(entire_op[0].to_owned());
            let operand2 = MonkeyId(entire_op[2].to_owned());
            if is_part2 && monkey_id.0 == *"root" {
                Root(operand1,operand2)
            }
            else {
                match entire_op[1] {
                    "+" => Add(operand1,operand2),
                    "-" => Sub(operand1,operand2),
                    "*" => Mul(operand1,operand2),
                    "/" => Div(operand1,operand2),
                    _ => panic!()
                }
            }
        };
        Monkey { id: monkey_id, op }
    }

    fn get_yelled_value(&self, table: &HashMap<MonkeyId,Monkey>, values: &mut HashMap<MonkeyId,i64>) -> i64 {
        if let Some(value) = values.get(&self.id) {
            *value
        }
        else {
            let value = match &self.op {
                Constant(v) => *v,
                Add(id1, id2) => table[id1].get_yelled_value(table,values) + table[id2].get_yelled_value(table,values),
                Sub(id1, id2) => table[id1].get_yelled_value(table,values) - table[id2].get_yelled_value(table,values),
                Mul(id1, id2) => table[id1].get_yelled_value(table,values) * table[id2].get_yelled_value(table,values),
                Div(id1, id2) => table[id1].get_yelled_value(table,values) / table[id2].get_yelled_value(table,values),
                _ => panic!()
            };
            values.insert(self.id.clone(),value);
            value
        }
    }

    fn construct_tree(&self, table: &HashMap<MonkeyId,Monkey>, values: &mut HashMap<MonkeyId,Option<i64>>) -> Option<i64> {
        if let Some(value) = values.get(&self.id) {
            *value
        }
        else {
            let value = match &self.op {
                Constant(v) => Some(*v),
                Variable => None,
                Add(id1, id2) => {
                    match (table[id1].construct_tree(table,values),table[id2].construct_tree(table,values)) {
                        (Some(u),Some(v)) => Some(u+v),
                        _ => None,
                    }
                } ,
                Sub(id1, id2) => {
                    match (table[id1].construct_tree(table,values),table[id2].construct_tree(table,values)) {
                        (Some(u),Some(v)) => Some(u-v),
                        _ => None,
                    }
                } ,
                Mul(id1, id2) => {
                    match (table[id1].construct_tree(table,values),table[id2].construct_tree(table,values)) {
                        (Some(u),Some(v)) => Some(u*v),
                        _ => None,
                    }
                } ,
                Div(id1, id2) => {
                    match (table[id1].construct_tree(table,values),table[id2].construct_tree(table,values)) {
                        (Some(u),Some(v)) => Some(u/v),
                        _ => None,
                    }
                } ,
                Root(id1,id2) => {
                    match (table[id1].construct_tree(table,values),table[id2].construct_tree(table,values)) {
                        (Some(u), None) => Some(u),
                        (None,Some(v)) => Some(v),
                        _ => panic!(),
                    }
                }
            };
            values.insert(self.id.clone(),value);
            value
        }
    }

    fn back_propagate(&self, table: &HashMap<MonkeyId,Monkey>, values: &HashMap<MonkeyId,Option<i64>>, curr_value: i64) -> i64 {
        match &self.op {
            Constant(_) => panic!(),
            Variable => curr_value,
            Add(id1, id2) => {
                match (values.get(id1).unwrap(),values.get(id2).unwrap()) {
                    (Some(u), None) => table[id2].back_propagate(table,values,curr_value - u),
                    (None, Some(u)) => table[id1].back_propagate(table,values,curr_value - u),
                    _ => panic!()
                }
            }
            Sub(id1, id2) => {
                match (values.get(id1).unwrap(),values.get(id2).unwrap()) {
                    (Some(u), None) => table[id2].back_propagate(table,values,u - curr_value),
                    (None, Some(u)) => table[id1].back_propagate(table,values,u + curr_value),
                    _ => panic!()
                }
            }
            Mul(id1, id2) => {
                match (values.get(id1).unwrap(),values.get(id2).unwrap()) {
                    (Some(u), None) => table[id2].back_propagate(table,values,curr_value / u),
                    (None, Some(u)) => table[id1].back_propagate(table,values,curr_value / u),
                    _ => panic!()
                }
            }
            Div(id1, id2) => {
                match (values.get(id1).unwrap(),values.get(id2).unwrap()) {
                    (Some(u), None) => table[id2].back_propagate(table,values,u / curr_value),
                    (None, Some(u)) => table[id1].back_propagate(table,values,curr_value * u),
                    _ => panic!()
                }
            }
            Root(id1, id2) => {
                match (values.get(id1).unwrap(),values.get(id2).unwrap()) {
                    (Some(u), None) => table[id2].back_propagate(table,values,*u),
                    (None, Some(u)) => table[id1].back_propagate(table,values,*u),
                    _ => panic!()
                }
            }
        }
    }
}

pub fn day21() {
    let mut file = File::open("./inputs/input_day21.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut monkey_table = HashMap::new();
    data.lines()
        .for_each(|s| {
            let monkey = Monkey::parse(s, false);
            monkey_table.insert(monkey.id.clone(),monkey);
        });

    let sol1 = monkey_table[&MonkeyId(String::from("root"))].get_yelled_value(&monkey_table,&mut HashMap::new());
    println!("Solution 1 : {:?}",sol1);

    let mut monkey_table2 = HashMap::new();
    data.lines()
        .for_each(|s| {
            let monkey = Monkey::parse(s,true);
            monkey_table2.insert(monkey.id.clone(),monkey);
        });

    let tree = &mut HashMap::new();
    monkey_table2[&MonkeyId(String::from("root"))].construct_tree(&monkey_table2,tree);
    //println!("Solution 2 : {:?}",tree);
    println!("Solution 2 : {:?}",monkey_table2[&MonkeyId(String::from("root"))].back_propagate(&monkey_table2,tree,0));
}