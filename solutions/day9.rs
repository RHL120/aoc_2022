use std::collections::HashSet;

#[derive(Debug)]
enum Motion {
    Left(u32),
    Right(u32),
    Up(u32),
    Down(u32),
}

struct Rope {
    tail: (i32, i32),
    head: (i32, i32),
}

impl Default for Rope {
    fn default() -> Self {
        Rope {
            tail: (0, 0),
            head: (0, 0),
        }
    }
}

impl Rope {
    fn touches_head(&self, nt: (i32, i32)) -> bool {
        (nt.0 - self.head.0).abs() <= 1 && (nt.1 - self.head.1).abs() <= 1
    }
    fn update_tail(&mut self) {
        if self.touches_head(self.tail) {
            return;
        }
        let ds = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        self.tail = ds
            .iter()
            .filter_map(|(dx, dy)| {
                let nt = (self.tail.0 + dx, self.tail.1 + dy);
                if self.touches_head(nt) {
                    Some(nt)
                } else {
                    None
                }
            })
            .min_by_key(|(x, y)| {
                let a = x - self.head.0;
                let b = y - self.head.1;
                a * a + b * b
            })
            .unwrap()
    }
}

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day9").ok()
}

fn part1_solve(instrucs: &Vec<Motion>) -> usize {
    let mut rope = Rope::default();
    let mut hs = HashSet::new();
    for motion in instrucs {
        let (dx, dy, count) = match motion {
            Motion::Left(x) => (-1, 0, x),
            Motion::Right(x) => (1, 0, x),
            Motion::Up(y) => (0, 1, y),
            Motion::Down(y) => (0, -1, y),
        };
        for _ in 0..*count {
            rope.head.0 += dx;
            rope.head.1 += dy;
            rope.update_tail();
            hs.insert(rope.tail);
        }
    }
    hs.len()
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
fn main() {
    let input = parse_input(&load_input().unwrap()).unwrap();
    println!("{:#?}", part1_solve(&input));
}
