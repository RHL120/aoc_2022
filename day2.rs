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

fn part2_parse_round(s: &str) -> Option<(Choices, States)> {
    let x: Vec<&str> = s.trim().split(" ").collect();
    Some((
        match x[0] {
            "A" => Choices::Rock,
            "B" => Choices::Paper,
            "C" => Choices::Scissors,
            _ => return None,
        },
        match x[1] {
            "X" => States::Loss,
            "Y" => States::Draw,
            "Z" => States::Victory,
            _ => return None,
        },
    ))
}

fn part2_parse(s: &str) -> Option<Vec<(Choices, States)>> {
    s.trim().split("\n").map(part2_parse_round).collect()
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

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day2").ok()
}

fn main() {
    if let Some(input) = load_input() {
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
