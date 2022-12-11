use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Item {
    worry_level: u64,
}

#[derive(Debug, Clone)]
enum OperationType {Plus, Mul}

#[derive(Debug, Clone)]
enum Operand {Value(u64), Arg}

#[derive(Debug, Clone)]
struct Operation {
    op_type: OperationType,
    operand: Operand,
}

#[derive(Debug, Clone, Copy)]
struct MonkeyId(usize);

#[derive(Debug, Clone)]
struct Monkey {
    held_items: Vec<Item>,
    op_worry: Operation,
    test_div: usize,
    monkey_true: MonkeyId,
    monkey_false: MonkeyId,
    nb_inspections: usize,
    mod_total: u64,
}

impl Monkey {
    fn play_turn(&mut self, new_rules: bool) -> Vec<(Item,MonkeyId)> {
        let thrown_items: Vec<(Item, MonkeyId)> = self.held_items.iter()
            .map(|i| {
                let new_item = self.apply_op(i, new_rules);
                let monkey_id = if new_item.worry_level % self.test_div as u64 == 0 {
                    self.monkey_true
                } else {
                    self.monkey_false
                };
                (new_item,monkey_id)
            }).collect();
        self.nb_inspections += self.held_items.len();
        self.held_items = Vec::new();
        thrown_items
    }

    fn apply_op(&self, item: &Item, new_rules: bool) -> Item {
        let operand = match self.op_worry.operand {
            Operand::Value(v) => v,
            Operand::Arg => item.worry_level
        };
        let divider = if new_rules {1} else {3};
        match self.op_worry.op_type {
            OperationType::Plus => Item { worry_level: ((item.worry_level + operand)/divider) % self.mod_total },
            OperationType::Mul  => Item { worry_level: ((item.worry_level * operand)/divider) % self.mod_total }
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        lines.next();
        //starting items
        let items = lines.next().unwrap();
        let held_items: Vec<Item> = items.split(": ")
            .nth(1).unwrap()
            .split(", ")
            .map(|nb| Item { worry_level: nb.parse::<u64>().unwrap() })
            .collect();
        //operation
        let op = lines.next().unwrap();
        let mut ops = op.split("= old ")
            .nth(1).unwrap()
            .split_whitespace();
        let op_type = match ops.next().unwrap() {
            "+" => OperationType::Plus,
            "*" => OperationType::Mul,
            _ => panic!()
        };
        let operand = match ops.next().unwrap() {
            "old" => Operand::Arg,
            s => Operand::Value(s.parse::<u64>().unwrap())
        };
        let op_worry = Operation { op_type, operand };
        //div test and who to throw to
        let mut iter_last_nums = lines.map(|s|
            s.split_whitespace()
            .last().unwrap()
            .parse::<usize>().unwrap());
        let test_div = iter_last_nums.next().unwrap();
        let monkey_true = MonkeyId(iter_last_nums.next().unwrap());
        let monkey_false = MonkeyId(iter_last_nums.next().unwrap());
        Ok(Monkey {
            held_items,
            op_worry,
            test_div,
            monkey_true,
            monkey_false,
            nb_inspections: 0,
            mod_total: 0
        })
    }
}

fn play_rounds(monkeys: &mut[Monkey], nb_rounds: usize, new_rules: bool) -> usize {
    let nb_monkeys = monkeys.len();
    for _ in 0..nb_rounds {
        for index in 0..nb_monkeys {
            let monkey = &mut monkeys[index];
            let thrown_items = monkey.play_turn(new_rules);
            for (item, monkey_id) in thrown_items.iter() {
                monkeys[monkey_id.0].held_items.push(*item);
            }
        }
    }
    let mut nb_inspections = monkeys.iter().map(|m| m.nb_inspections).collect::<Vec<usize>>();
    nb_inspections.sort_unstable();
    nb_inspections[nb_monkeys-1] * nb_inspections[nb_monkeys-2]
}

pub fn day11() {
    let mut file = File::open("./inputs/input_day11.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let mut monkeys: Vec<Monkey> = data.split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();

    let mod_total: usize = monkeys.iter()
        .map(|m| m.test_div)
        .product();

    monkeys.iter_mut()
        .for_each(|m| m.mod_total = mod_total as u64);

    let mut monkeys_new_rules = monkeys.clone();

    let nb_rounds_b = 20;
    let nb_rounds_n = 10000;

    let monkey_business = play_rounds(monkeys.as_mut_slice(),nb_rounds_b,false);
    let monkey_business_new = play_rounds(monkeys_new_rules.as_mut_slice(),nb_rounds_n,true);

    println!("Solution 1 : {}",monkey_business);
    println!("Solution 2 : {}",monkey_business_new);
}