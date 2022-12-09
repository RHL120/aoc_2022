#[derive(Debug)]
enum Motion {
    Left(u32),
    Right(u32),
    Up(u32),
    Down(u32),
}
fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day9").ok()
}

fn parse_input(input: &str) -> Option<Vec<Motion>> {
    input
        .trim()
        .split("\n")
        .map(|x| {
            let args = x.split(" ").collect::<Vec<&str>>();
            Some(match *args.get(0)? {
                "L" => Motion::Left(args.get(1)?.parse().ok()?),
                "R" => Motion::Right(args.get(1)?.parse().ok()?),
                "U" => Motion::Up(args.get(1)?.parse().ok()?),
                "D" => Motion::Down(args.get(1)?.parse().ok()?),
                _ => return None,
            })
        })
        .collect()
}
fn are_neighbors((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> bool {
    (ax - bx).abs() <= 1 && (ay - by).abs() <= 1
}

fn update_rope(rope: &mut Vec<(i32, i32)>, idx: usize) {
    if idx >= rope.len() || rope[idx] == rope[idx - 1] || are_neighbors(rope[idx], rope[idx - 1]) {
        return;
    }
    rope[idx] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ]
    .iter()
    .filter_map(|(dx, dy)| {
        let new_knot = (rope[idx].0 + dx, rope[idx].1 + dy);
        if are_neighbors(new_knot, rope[idx - 1]) {
            Some(new_knot)
        } else {
            None
        }
    })
    .min_by_key(|(x, y)| {
        let a = x - rope[idx - 1].0;
        let b = y - rope[idx - 1].1;
        a * a + b * b
    })
    .unwrap();
    update_rope(rope, idx + 1);
}

fn move_rope(rope: &mut Vec<(i32, i32)>, instructions: &Vec<Motion>) -> usize {
    let mut visited = std::collections::HashSet::new();
    for instruction in instructions {
        let (dx, dy, count) = match instruction {
            Motion::Left(x) => (-1, 0, x),
            Motion::Right(x) => (1, 0, x),
            Motion::Up(y) => (0, 1, y),
            Motion::Down(y) => (0, -1, y),
        };
        for _ in 0..*count {
            rope[0].0 += dx;
            rope[0].1 += dy;
            update_rope(rope, 1);
            visited.insert(rope[rope.len() - 1]);
        }
    }
    visited.len()
}

fn main() {
    if let Some(input) = load_input() {
        if let Some(input) = parse_input(&input) {
            let mut rope1 = Vec::new();
            let mut rope2 = Vec::new();
            for _ in 0..2 {
                rope1.push((0, 0));
            }
            for _ in 0..10 {
                rope2.push((0, 0));
            }
            println!(
                "The solution to part1 is: {}",
                move_rope(&mut rope1, &input)
            );
            println!(
                "The solution to part2 is: {}",
                move_rope(&mut rope2, &input)
            );
        } else {
            println!("Failed to parse input file");
        }
    } else {
        println!("Failed to read the input file");
    }
}
