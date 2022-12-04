#[derive(Debug, Copy, Clone)]
struct Assignment(u8, u8);
#[derive(Debug, Copy, Clone)]

struct AssignmentPair(Assignment, Assignment);

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day4").ok()
}

fn parse_input(s: &str) -> Vec<AssignmentPair> {
    s.trim()
        .split("\n")
        .map(|x| {
            let x: Vec<Assignment> = x
                .trim()
                .split(",")
                .map(|x| {
                    let x: Vec<&str> = x.trim().split("-").collect();
                    Assignment(x[0].parse().unwrap(), x[1].parse().unwrap())
                })
                .collect();
            AssignmentPair(x[0], x[1])
        })
        .collect()
}

fn overlaps(
    AssignmentPair(Assignment(start1, end1), Assignment(start2, end2)): AssignmentPair,
) -> bool {
    if (start1 >= start2 && end1 <= end2) || (start1 <= start2 && end1 >= end2) {
        true
    } else {
        false
    }
}

fn main() {
    let input = load_input().unwrap();
    println!(
        "{:#?}",
        parse_input(&input)
            .iter()
            .filter(|x| overlaps(*x.clone()))
            .collect::<Vec<&AssignmentPair>>()
            .len()
    );
}
