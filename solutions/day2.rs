#[derive(Clone, Copy, Debug)]
enum Choices {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, Debug)]
enum States {
    Loss,
    Draw,
    Victory,
}

fn parse_choice(x: &str) -> Option<Choices> {
    Some(match x {
        "A" => Choices::Rock,
        "B" => Choices::Paper,
        "C" => Choices::Scissors,
        c => match c {
            "X" => Choices::Rock,
            "Y" => Choices::Paper,
            "Z" => Choices::Scissors,
            _ => return None,
        },
    })
}

fn parse_state(x: &str) -> Option<States> {
    Some(match x {
        "X" => States::Loss,
        "Y" => States::Draw,
        "Z" => States::Victory,
        _ => return None,
    })
}

fn part2_parse_round(s: &str) -> Option<(Choices, States)> {
    let x: Vec<&str> = s.trim().split(" ").collect();
    Some((parse_choice(x[0])?, parse_state(x[1])?))
}

fn part2_parse(s: &str) -> Option<Vec<(Choices, States)>> {
    s.trim().split("\n").map(part2_parse_round).collect()
}

fn part1_parse_round(s: &str) -> Option<(Choices, Choices)> {
    let x: Vec<&str> = s.trim().split(" ").collect();
    Some((parse_choice(x[0])?, parse_choice(x[1])?))
}

fn part1_parse(s: &str) -> Option<Vec<(Choices, Choices)>> {
    s.trim().split("\n").map(part1_parse_round).collect()
}

fn part2_calc_score((choice, state): (Choices, States)) -> u32 {
    use Choices::*;
    use States::*;
    match state {
        Victory => match choice {
            Rock => 8,     //He chose rock, I chose paper
            Paper => 9,    //He chose Paper, I chose Scissors
            Scissors => 7, //He chose Scissors, I chose rock
        },
        Loss => match choice {
            Rock => 3,     //He chose Rock, I chose Scissors
            Paper => 1,    //He chose Paper I chose Rock
            Scissors => 2, //He chose Scissors, I chose Paper
        },
        Draw => match choice {
            Rock => 4,     // Rocks
            Paper => 5,    // Paper
            Scissors => 6, // Scissors
        },
    }
}

fn part1_calc_score((c1, c2): (Choices, Choices)) -> u32 {
    use Choices::*;
    match c1 {
        Rock => match c2 {
            Rock => 4,     //he chose rock I chose rock
            Paper => 8,    //he chose rock I chose paper
            Scissors => 3, //he chose rock I chose scissors
        },
        Paper => match c2 {
            Rock => 1,     //he chose paper I chose rock
            Paper => 5,    //he chose paper I chose paper
            Scissors => 9, //he chose paper I chose scissors
        },
        Scissors => match c2 {
            Rock => 7,
            Paper => 2,
            Scissors => 6,
        },
    }
}

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day2").ok()
}

fn main() {
    if let Some(input) = load_input() {
        if let Some(input) = part1_parse(&input) {
            println!(
                "the solution to part 1 is: {}",
                input
                    .iter()
                    .map(|x| part1_calc_score(x.clone()))
                    .sum::<u32>()
            );
        } else {
            println!("Failed to parse the input file");
        }
        if let Some(input) = part2_parse(&input) {
            println!(
                "the solution to part 2 is: {}",
                input
                    .iter()
                    .map(|x| part2_calc_score(x.clone()))
                    .sum::<u32>()
            );
        } else {
            println!("Failed to parse the input file");
        }
    } else {
        println!("Failed to read the input file");
    }
}
