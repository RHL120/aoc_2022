fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day3").ok()
}

fn comparts(input: &str) -> Vec<(String, String)> {
    input
        .split("\n")
        .map(|input| {
            let input = input.trim();
            (
                input[..input.len() / 2].to_string(),
                input[input.len() / 2..].to_string(),
            )
        })
        .collect()
}

fn part1_solve((c1, c2): &(String, String)) -> usize {
    let mut res = 0;
    for i in 65..91 {
        let chr = char::from_u32(i as u32).unwrap();
        if c1.contains(chr) && c2.contains(chr) {
            res += i - 38;
        }
    }
    for i in 97..123 {
        let chr = char::from_u32(i as u32).unwrap();
        if c1.contains(chr) && c2.contains(chr) {
            res += i - 96;
        }
    }
    res
}

fn part2_solve(groups: Vec<Vec<&str>>) -> u32 {
    groups
        .iter()
        .map(|x| {
            let mut ret = 0;
            for i in x[0].chars() {
                if x[1].contains(i) && x[2].contains(i) {
                    let ord: u32 = i.into();
                    ret += ord - if i.is_uppercase() { 38 } else { 96 };
                    break;
                }
            }
            ret
        })
        .sum()
}

fn main() {
    if let Some(input) = load_input() {
        println!("[*] Loaded input");
        let c = comparts(input.trim());
        println!(
            "[*] The solution to part one is: {}",
            c.iter().map(part1_solve).sum::<usize>()
        );
        println!(
            "The solution to part 2 is: {}",
            part2_solve(
                input
                    .trim()
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .chunks(3)
                    .map(Vec::from)
                    .collect()
            )
        );
    } else {
        println!("Failed to load input");
    }
}
