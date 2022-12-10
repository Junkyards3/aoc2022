use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum InstructionType {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
struct Instruction {
    instr_type: InstructionType,
    remaining_cycles: usize,
}

impl Instruction {
    fn new_noop() -> Self {
        Instruction {
            instr_type: InstructionType::Noop,
            remaining_cycles: 1,
        }
    }

    fn new_addx(value: i32) -> Self {
        Instruction {
            instr_type: InstructionType::Addx(value),
            remaining_cycles: 2,
        }
    }

    fn execute_cycle(&mut self) -> (bool,Option<i32>) {
        self.remaining_cycles -= 1;
        if self.remaining_cycles == 0 {
            return match self.instr_type {
                InstructionType::Noop => (true, None),
                InstructionType::Addx(v) => (true, Some(v))
            }
        }
        (false,None)
    }

}

struct CPU {
    nb_cycles: usize,
    instr_stack : Vec<Instruction>,
    value_x : i32,
}

impl CPU {
    fn new(instrs: impl Iterator<Item = Instruction>) -> Self {
        CPU {
            nb_cycles: 1,
            instr_stack: instrs.collect(),
            value_x: 1,
        }
    }
    fn execute_cycle(&mut self) -> Option<i32> {
        self.nb_cycles += 1;
        if let Some(instr) = self.instr_stack.last_mut() {
            let (ended,result) = instr.execute_cycle();
            if ended {
                self.instr_stack.pop();
                if let Some(value_to_add) = result {
                    self.value_x += value_to_add;
                }
            }
            return None;
        }
        Some(self.value_x)
    }

    fn signal_strength(&self) -> i32 {
        self.nb_cycles as i32 * self.value_x
    }

    fn is_sprite_visible(&self) -> bool {
        let screen_px = (self.nb_cycles % 40) as i32;
        screen_px >= self.value_x && screen_px <= self.value_x + 2
    }
}

pub fn day10() {
    let mut file = File::open("./inputs/input_day10.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let iter = data.split('\n').
        map(|line| {
            let mut it = line.split_whitespace();
            if it.next().unwrap() == "noop" {
                Instruction::new_noop()
            }
            else {
                Instruction::new_addx(it.next().unwrap().parse::<i32>().unwrap())
            }
        });
    let mut cpu = CPU::new(iter.rev());
    let cycles: Vec<usize> = vec![20,60,100,140,180,220];
    let mut result1: i32 = 0;
    let mut result2 = String::from("");
    match cpu.is_sprite_visible() {
        true => result2.push('#'),
        false => result2.push('.')
    }
    while cpu.execute_cycle().is_none() {
        match cpu.is_sprite_visible() {
            true => result2.push('#'),
            false => result2.push('.')
        }
        if cpu.nb_cycles % 40 == 0 {
            result2.push('\n');
        }
        if cycles.contains(&cpu.nb_cycles) {
            result1 += cpu.signal_strength();
        }
    }
    println!("Solution 1 : {:?}", result1);
    println!("Solution 2 : \n{}", result2);
}