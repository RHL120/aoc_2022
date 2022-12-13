use std::cmp::{Eq, Ord, Ordering, PartialEq};
use std::fmt::Display;
use std::str::FromStr;
#[derive(PartialEq, Eq, Clone)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

//We can't use Ordering because Equality is not a useful construct for Packet
//(in terms of this puzzle)
impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(i) => i.fmt(f),
            Packet::List(l) => {
                write!(f, "[")?;
                for (idx, elem) in l.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?;
                    }
                    elem.fmt(f)?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
enum ParserError {
    InvalidList(String),
    InvalidInt(String),
    Unknown(String),
}

fn parse_int<'a>(input: &'a str) -> Result<(Packet, &'a str), ParserError> {
    let ret: String = input.chars().take_while(|x| x.is_digit(10)).collect();
    Ok((
        Packet::Int(ret.parse().or(Err(ParserError::InvalidInt(ret.clone())))?),
        input
            .strip_prefix(&ret)
            .ok_or(ParserError::InvalidInt(ret))?,
    ))
}

fn parse_list<'a>(input: &'a str) -> Result<(Packet, &'a str), ParserError> {
    let mut rest = input
        .strip_prefix('[')
        .ok_or(ParserError::InvalidList(input.to_string()))?;
    let mut ret = Vec::new();
    while let Ok((elem, r)) = parse_packet(rest) {
        rest = r.strip_prefix(",").unwrap_or(r);
        ret.push(elem);
    }
    Ok((
        Packet::List(ret),
        rest.strip_prefix(']')
            .ok_or(ParserError::InvalidList(rest.to_string()))?,
    ))
}

fn parse_packet<'a>(input: &'a str) -> Result<(Packet, &'a str), ParserError> {
    parse_int(input).or(parse_list(input))
}

impl FromStr for Packet {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_packet(s).and_then(|(ret, rest)| {
            if rest != "" {
                Err(ParserError::Unknown(rest.to_string()))
            } else {
                Ok(ret)
            }
        })
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use std::cmp::Ordering::*;
        use Packet::*;
        Some(match self {
            Int(i1) => match other {
                Int(i2) => i1.cmp(i2),
                a @ List(_) => List(vec![Int(*i1)]).cmp(a),
            },
            a @ List(l1) => match other {
                Int(i2) => a.cmp(&List(vec![Int(*i2)])),
                List(l2) => {
                    let mut ret = l1.len().cmp(&l2.len());
                    let mut i = 0;
                    while i < l1.len() && i < l2.len() {
                        match l1[i].cmp(&l2[i]) {
                            Equal => (),
                            x => {
                                ret = x;
                                break;
                            }
                        };
                        i += 1;
                    }
                    ret
                }
            },
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day13").ok()
}

fn part1_solve(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, x)| {
            let (a, b) = x
                .split_once("\n")
                .ok_or(ParserError::Unknown(x.to_string()))
                .unwrap();
            let (a, b): (Packet, Packet) = (a.parse().unwrap(), b.parse().unwrap());
            if a < b {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2_solve(input: &str) -> usize {
    use Packet::*;
    let mut a = input
        .trim()
        .replace("\n\n", "\n")
        .split("\n")
        .map(|x| x.parse())
        .collect::<Result<Vec<Packet>, ParserError>>()
        .unwrap();
    let div1 = List(vec![List(vec![Int(2)])]);
    let div2 = List(vec![List(vec![Int(6)])]);
    a.push(div1.clone());
    a.push(div2.clone());
    a.sort();
    (a.iter().position(|x| x == &div1).unwrap() + 1)
        * (a.iter().position(|x| x == &div2).unwrap() + 1)
}

fn main() {
    let input = load_input().unwrap();
    println!("The solution to part1 is: {}", part1_solve(&input));
    println!("The solution to part2 is: {}", part2_solve(&input));
}
