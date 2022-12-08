fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day8").ok()
}

fn is_visible(grid: &Vec<Vec<u32>>, x_len: usize, x: usize, y: usize) -> bool {
    x == 0
        || x == x_len - 1
        || (0..y).all(|i| grid[i][x] < grid[y][x])
        || (y + 1..grid.len()).all(|i| grid[i][x] < grid[y][x])
        || (0..x).all(|i| grid[y][i] < grid[y][x])
        || (x + 1..grid.len()).all(|i| grid[y][i] < grid[y][x])
}

fn solve_part1(input: &str) -> Option<usize> {
    let grid = input
        .trim()
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|x| {
                    if let Some(y) = x.to_digit(10) {
                        Some(y)
                    } else {
                        println!("Bad char: {}", x);
                        None
                    }
                })
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()?;
    let x_len = grid.get(0)?.len();
    let mut ret = 0;
    for (y, row) in grid.iter().enumerate() {
        if y == 0 || y == grid.len() - 1 {
            ret += x_len;
        } else {
            if row.len() != x_len {
                return None;
            }
            for (x, _) in row.iter().enumerate() {
                if is_visible(&grid, x_len, x, y) {
                    ret += 1
                }
            }
        }
    }
    return Some(ret);
}

fn score(grid: &Vec<Vec<u32>>, x_len: usize, x: usize, y: usize) -> u32 {
    let mut top = 0;
    let mut bottom = 0;
    let mut left = 0;
    let mut right = 0;
    for i in (0..y).rev() {
        top += 1;
        if grid[i][x] >= grid[y][x] {
            break;
        }
    }
    for i in y + 1..grid.len() {
        bottom += 1;
        if grid[i][x] >= grid[y][x] {
            break;
        }
    }
    for i in (0..x).rev() {
        left += 1;
        if grid[y][i] >= grid[y][x] {
            break;
        }
    }
    for i in x + 1..grid.len() {
        right += 1;
        if grid[y][i] >= grid[y][x] {
            break;
        }
    }
    return top * bottom * left * right;
}

fn part2_solve(input: &str) -> Option<u32> {
    let grid = input
        .trim()
        .split("\n")
        .map(|x| {
            x.chars()
                .map(|x| {
                    if let Some(y) = x.to_digit(10) {
                        Some(y)
                    } else {
                        println!("Bad char: {}", x);
                        None
                    }
                })
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()?;
    let mut max = 0;
    let x_len = grid.get(0)?.len();
    for (y, row) in grid.iter().enumerate() {
        if row.len() != x_len {
            return None;
        }
        for (x, _) in row.iter().enumerate() {
            let s = score(&grid, x_len, x, y);
            if s > max {
                max = s
            }
        }
    }
    return Some(max);
}

fn main() {
    if let Some(input) = load_input() {
        if let Some(solution) = solve_part1(&input) {
            println!("The solution to part 1 is: {}", solution)
        } else {
            eprintln!("Failed to solve part 1")
        }
        if let Some(solution) = part2_solve(&input) {
            println!("{}", solution)
        } else {
            eprintln!("Failed to solve part 2")
        }
    } else {
        eprintln!("Failed to load input")
    }
}
