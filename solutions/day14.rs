use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

struct Path(Vec<(isize, isize)>);
#[derive(Debug)]
pub struct Grid {
    rocks: HashSet<(isize, isize)>,
    sand: HashSet<(isize, isize)>,
    abyss_y: isize,
}

impl FromStr for Path {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(
            s.trim()
                .split(" -> ")
                .map(|s| {
                    let (x, y) = s.split_once(",").ok_or(())?;
                    Ok((x.parse().or(Err(()))?, y.parse().or(Err(()))?))
                })
                .collect::<Result<Vec<(isize, isize)>, Self::Err>>()?,
        ))
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, (x, y)) in self.0.iter().enumerate() {
            if idx != 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{},{}", x, y)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(input: &Vec<Path>) -> Self {
        let mut rocks = HashSet::new();
        let sand = HashSet::new();
        let mut abyss_y = 0;
        for Path(path) in input {
            for (idx, point) in path.iter().enumerate() {
                let (cx, cy) = *point;
                if idx < path.len() - 1 {
                    let (nx, ny) = path[idx + 1];
                    let (fx, fy) = (min(cx, nx), min(cy, ny));
                    let (lx, ly) = (max(cx, nx), max(cy, ny));
                    (fx..lx + 1).for_each(|x| {
                        rocks.insert((x, fy));
                    });
                    (fy..ly + 1).for_each(|y| {
                        abyss_y = max(abyss_y, y);
                        rocks.insert((fx, y));
                    });
                }
            }
        }
        println!("{}", abyss_y);
        Grid {
            rocks,
            sand,
            abyss_y,
        }
    }
    fn contains(&self, p: (isize, isize)) -> bool {
        self.rocks.contains(&p) || self.sand.contains(&p)
    }
    fn is_at_rest(&self, (x, y): (isize, isize)) -> bool {
        [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
            .iter()
            .all(|x| self.contains(*x))
    }
    fn new_sand(&mut self) -> bool {
        let mut p = (500, 1);
        while !self.is_at_rest(p) {
            let (x, y) = p;
            if y > self.abyss_y {
                return true;
            }
            p = *[(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)]
                .iter()
                .find(|x| !self.contains(**x))
                .unwrap();
        }
        println!("here: ({}, {})", p.0, p.1);
        assert!(self.sand.insert(p));
        return false;
    }
}

fn part1_solve(grid: &mut Grid) -> usize {
    let mut i = 0;
    while !grid.new_sand() {
        i += 1;
    }
    i
}

fn parse_input(input: &str) -> Option<Vec<Path>> {
    input.trim().split("\n").map(|x| x.parse().ok()).collect()
}

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day14").ok()
}

fn main() {
    let input = load_input().and_then(|x| parse_input(&x)).unwrap();
    let mut grid = Grid::new(&input);
    println!("The solution to part 1 is: {}", part1_solve(&mut grid));
}
