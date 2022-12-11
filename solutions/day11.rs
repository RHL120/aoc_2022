use std::str::FromStr;

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day11").ok()
}

type Item = usize;

struct Block {
    monkey_idx: usize,
    items: Vec<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    test: usize,
    on_true: usize,
    on_false: usize,
    counter: u32,
}

fn parse_idx(line: &str) -> Option<usize> {
    Some(
        line.strip_prefix("Monkey ")?
            .strip_suffix(":")?
            .parse()
            .ok()?,
    )
}

fn parse_items(line: &str) -> Option<Vec<Item>> {
    Some(
        line.strip_prefix("  Starting items: ")?
            .split(",")
            .map(|x| x.trim().parse().ok())
            .collect::<Option<Vec<Item>>>()?,
    )
}

fn parse_operation(line: &str) -> Option<Box<dyn Fn(Item) -> Item>> {
    let op = line.strip_prefix("  Operation: new = old ")?;
    if let Some(n) = op.strip_prefix("+ ") {
        if n == "old" {
            Some(Box::new(move |x| x + x))
        } else {
            let n = n.parse::<usize>().ok()?;
            Some(Box::new(move |x| x + n))
        }
    } else if let Some(n) = op.strip_prefix("* ") {
        if n == "old" {
            Some(Box::new(move |x| x * x))
        } else {
            let n = n.parse::<usize>().ok()?;
            Some(Box::new(move |x| x * n))
        }
    } else {
        None
    }
}

fn parse_on_true(line: &str) -> Option<usize> {
    Some(
        line.strip_prefix("    If true: throw to monkey ")?
            .parse()
            .ok()?,
    )
}
fn parse_on_false(line: &str) -> Option<usize> {
    Some(
        line.strip_prefix("    If false: throw to monkey ")?
            .parse()
            .ok()?,
    )
}

fn parse_test(line: &str) -> Option<usize> {
    let n = line
        .strip_prefix("  Test: divisible by ")?
        .parse::<usize>()
        .ok()?;
    Some(n)
}

fn parse_input(input: &str) -> Option<Vec<Block>> {
    let mut ret = Vec::new();
    for i in input.split("\n\n") {
        let i = i.lines().collect::<Vec<&str>>();
        ret.push(Block {
            monkey_idx: parse_idx(i[0])?,
            items: parse_items(i[1])?,
            operation: parse_operation(i[2])?,
            test: parse_test(i[3])?,
            on_true: parse_on_true(i[4])?,
            on_false: parse_on_false(i[5])?,
            counter: 0,
        });
    }
    Some(ret)
}

fn solve_pt1(blocks: Vec<Block>) -> u32 {
    let mut blocks = blocks;
    for _ in 0..20 {
        for bidx in 0..blocks.len() {
            while blocks[bidx].items.len() > 0 {
                blocks[bidx].counter += 1;
                let new = (blocks[bidx].operation)(blocks[bidx].items[0]) / 3;
                if new % blocks[bidx].test == 0 {
                    let idx = blocks[bidx].on_true;
                    blocks[idx].items.push(new);
                } else {
                    let idx = blocks[bidx].on_false;
                    blocks[idx].items.push(new);
                }
                blocks[bidx].items.remove(0);
            }
        }
    }
    blocks.sort_by_key(|x| x.counter);
    blocks[blocks.len() - 1].counter * blocks[blocks.len() - 2].counter
}

fn main() {
    let input = load_input().unwrap();
    println!(
        "The solution to part 1 is: {}",
        solve_pt1(parse_input(&input).unwrap())
    );
}
