fn parse_input(data: &str) -> Vec<i32> {
    let v = data
        .split("\n\n")
        .map(|x| {
            x.split("\n").fold(0, |x, y| match y.trim().parse::<i32>() {
                Ok(y) => x + y,
                _ => x,
            })
        })
        .collect();
    return v;
}

fn load_input() -> String {
    std::fs::read_to_string("./inputs/day1").unwrap()
}

fn main() {
    let p = parse_input(&load_input());
    println!("The solution for part 1 is {}", p.iter().max().unwrap());
}
