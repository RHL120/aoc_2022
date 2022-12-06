fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day6").ok()
}

fn find_marker1(input: &str) -> Option<(String, usize)> {
    let mut idx = 0;
    while idx < input.len() {
        if idx >= 4 {
            let ret = &input[idx - 4..idx];
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

fn find_marker2(input: &str) -> Option<(String, usize)> {
    let mut idx = 0;
    while idx < input.len() {
        if idx >= 14 {
            let ret = &input[idx - 14..idx];
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
        find_marker1(&input).unwrap()
    );
    println!(
        "The solution to part2 is: {:#?}",
        find_marker2(&input).unwrap()
    );
}
