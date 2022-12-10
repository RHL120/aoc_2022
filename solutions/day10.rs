#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

pub struct Cpu<'a> {
    started_add: bool,
    cycle_counter: usize,
    x_reg: i32,
    idx: usize,
    instructions: &'a Vec<Instruction>,
}

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day10").ok()
}

fn parse_input(input: &str) -> Option<Vec<Instruction>> {
    input
        .trim()
        .split("\n")
        .map(|x| {
            if x == "noop" {
                Some(Instruction::Noop)
            } else if x.starts_with("addx ") {
                Some(Instruction::Addx(x.replace("addx ", "").parse().ok()?))
            } else {
                println!("Error on line: {}", x);
                None
            }
        })
        .collect()
}

impl<'a> Cpu<'a> {
    fn new(instrucs: &'a Vec<Instruction>) -> Self {
        Cpu {
            started_add: false,
            cycle_counter: 1,
            x_reg: 1,
            idx: 0,
            instructions: instrucs,
        }
    }
}

impl<'a> Iterator for Cpu<'a> {
    type Item = (usize, i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.instructions.len() {
            return None;
        }
        self.cycle_counter += 1;
        match self.instructions[self.idx] {
            Instruction::Noop => {
                self.idx += 1;
            }
            Instruction::Addx(x) => {
                if self.started_add {
                    self.x_reg += x;
                    self.idx += 1;
                    self.started_add = false;
                } else {
                    self.started_add = true;
                }
            }
        };
        Some((self.cycle_counter, self.x_reg))
    }
}

fn main() {
    if let Some(input) = load_input() {
        if let Some(instructions) = parse_input(&input) {
            let cpu = Cpu::new(&instructions);
            let mut s = 0;
            for (i, x_reg) in cpu {
                if [20, 60, 100, 140, 180, 220].contains(&i) {
                    println!("{} * {} = {}", i, x_reg, (i as isize) * (x_reg as isize));
                    s += (i as isize) * (x_reg as isize);
                }
            }
            println!("The solution to part1 is: {}", s);
        } else {
            println!("Failed to parse input");
        }
    } else {
        println!("Failed to load input");
    }
}
