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
    let mut p = parse_input(&load_input());
    p.sort();
    println!("The solution for part 1 is {}", p[p.len() - 1]);
    println!(
        "The solution for part 2 is {}",
        p[p.len() - 1] + p[p.len() - 2] + p[p.len() - 3]
    );
}
