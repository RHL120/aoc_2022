fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day6").ok()
}

fn find_marker(input: &str, n_letters: usize) -> Option<(String, usize)> {
    let mut idx = 0;
    while idx < input.len() {
        if idx >= n_letters {
            let ret = &input[idx - n_letters..idx];
            let mut is_starter = true;
            for chr in ret.chars() {
                if ret.chars().filter(|x| x.clone() == chr).count() > 1 {
                    is_starter = false;
                    break;
                }
            }
            if is_starter {
                return Some((ret.to_string(), idx));
            }
        }
        idx += 1;
    }
    return None;
}

fn main() {
    let input = load_input().unwrap();
    println!(
        "The solution to part1 is: {:#?}",
        find_marker(&input, 4).unwrap()
    );
    println!(
        "The solution to part2 is: {:#?}",
        find_marker(&input, 14).unwrap()
    );
}
