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
    if let Some((marker, solution)) = find_marker(&input, 4) {
        println!(
            "The marker for part 1 is \"{}\" and the solution is: {}",
            marker, solution
        );
    }
    if let Some((marker, solution)) = find_marker(&input, 14) {
        println!(
            "The marker for part 2 is \"{}\"and the solution is: {}",
            marker, solution
        );
    }
}
