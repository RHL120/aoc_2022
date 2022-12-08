fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day8").ok()
}

fn is_visible(grid: &Vec<Vec<u32>>, x_len: usize, x: usize, y: usize) -> bool {
    let mut top = true;
    let mut bottom = true;
    let mut left = true;
    let mut right = true;
    if y == 0 || y == grid.len() - 1 || x == 0 || x == x_len - 1 {
        return true;
    }
    for i in 0..y {
        if grid[i][x] >= grid[y][x] {
            top = false;
            break;
        }
    }
    for i in y + 1..grid.len() {
        if grid[i][x] >= grid[y][x] {
            bottom = false;
            break;
        }
    }
    for i in 0..x {
        if grid[y][i] >= grid[y][x] {
            left = false;
            break;
        }
    }
    for i in x + 1..grid.len() {
        if grid[y][i] >= grid[y][x] {
            right = false;
            break;
        }
    }
    return top || bottom || left || right;
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
        if row.len() != x_len {
            return None;
        }
        for (x, _) in row.iter().enumerate() {
            /*if x == 0
                || x == x_len - 1
                || grid.iter().all(|r| r[x] <= *cell)
                || row.iter().max()? == cell
            {
                ret += 1
            }*/
            if is_visible(&grid, x_len, x, y) {
                ret += 1
            }
        }
    }
    return Some(ret);
}

fn main() {
    if let Some(input) = load_input() {
        if let Some(solution) = solve_part1(&input) {
            println!("{}", solution)
        } else {
            eprintln!("Failed to solve the problem")
        }
    } else {
        eprintln!("Failed to load input")
    }
}
