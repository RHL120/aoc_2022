type Crate = char;
type Stack = Vec<Crate>;
struct Instruction {
    quant: usize,
    dst: usize,
    src: usize,
}
fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day5").ok()
}

fn split_input(input: &str) -> Option<(String, String)> {
    let mut input = input.split("\n\n");
    Some((input.next()?.to_string(), input.next()?.to_string()))
}

//I just realized that all I could have just checked with the numbers on the last
//line but of course I was too stupid to notice it before.
fn parse_stack(input: &str) -> Option<Vec<Stack>> {
    let mut ret = Vec::new();
    let mut stacks = input.split("\n").collect::<Vec<&str>>();
    stacks.pop();
    for i in stacks {
        let mut stack_idx = 0;
        let mut n_chars = 0;
        let mut parsing_crate = false;
        for i in i.chars() {
            if n_chars % 4 == 0 && n_chars != 0 {
                stack_idx += 1;
                n_chars = 0;
            }
            if parsing_crate {
                if i.is_ascii_uppercase() {
                    while ret.len() <= stack_idx {
                        ret.push(Vec::new());
                    }
                    ret[stack_idx].push(i);
                } else if i == ']' {
                    parsing_crate = false;
                } else {
                    return None;
                }
            } else if i == '[' {
                parsing_crate = true;
            } else if i != ' ' {
                return None;
            }
            n_chars += 1;
        }
    }
    Some(ret)
}

fn parse_instructions(input: &str) -> Option<Vec<Instruction>> {
    input
        .trim()
        .split("\n")
        .map(|x| {
            let v = x
                .replace("move ", "")
                .replace(" from ", " ")
                .replace(" to ", " ")
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            Some(Instruction {
                quant: v.get(0)?.parse::<usize>().ok()?,
                src: v.get(1)?.parse::<usize>().ok()? - 1,
                dst: v.get(2)?.parse::<usize>().ok()? - 1,
            })
        })
        .collect::<Option<Vec<Instruction>>>()
}

fn part1_solve(stacks: &Vec<Stack>, instructs: &Vec<Instruction>) -> Option<String> {
    let mut stacks = stacks.clone();
    for i in instructs {
        for _ in 0..i.quant {
            let a = stacks.get(i.src)?.get(0)?.clone();
            stacks.get_mut(i.dst)?.insert(0, a);
            stacks.get_mut(i.src)?.remove(0);
        }
    }
    stacks
        .iter()
        .map(|x| Some(x.get(0)?.to_owned()))
        .collect::<Option<String>>()
}

fn main() {
    if let Some(input) = load_input().and_then(|x| split_input(&x)) {
        if let Some(instrucs) = parse_instructions(&input.1) {
            if let Some(stacks) = parse_stack(&input.0) {
                println!(
                    "The solution to part 1 is: {}",
                    part1_solve(&stacks, &instrucs).unwrap()
                );
            }
        } else {
            eprintln!("Failed to parse instructions");
        }
    } else {
        eprintln!("Failed to load input");
    }
}
