use std::fmt::Display;
use std::str::FromStr;
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

//We can't use Ordering because Equality is not a useful construct for Packet
//(in terms of this puzzle)
#[derive(Debug)]
enum PacketOrd {
    Less,
    Greater,
    Ndf,
}

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

impl Packet {
    fn pcmp(&self, other: &Self) -> PacketOrd {
        use std::cmp::Ordering::*;
        use Packet::*;
        match self {
            Int(i1) => match other {
                Int(i2) => match i1.cmp(i2) {
                    Greater => PacketOrd::Greater,
                    Less => PacketOrd::Less,
                    Equal => PacketOrd::Ndf,
                },
                a @ List(_) => List(vec![Int(*i1)]).pcmp(a),
            },
            a @ List(l1) => match other {
                Int(i2) => a.pcmp(&List(vec![Int(*i2)])),
                List(l2) => {
                    let mut ret = match l1.len().cmp(&l2.len()) {
                        Greater => PacketOrd::Greater,
                        Less => PacketOrd::Less,
                        Equal => PacketOrd::Ndf,
                    };
                    let mut i = 0;
                    while i < l1.len() && i < l2.len() {
                        match l1[i].pcmp(&l2[i]) {
                            PacketOrd::Ndf => (),
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
        }
    }
}

fn load_input() -> Option<String> {
    std::fs::read_to_string("./inputs/day13").ok()
}

fn main() {
    let input = load_input().unwrap();
    let a = input
        .trim()
        .split("\n\n")
        .map(|x| {
            let (a, b) = x
                .split_once("\n")
                .ok_or(ParserError::Unknown(x.to_string()))?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<Vec<(Packet, Packet)>, ParserError>>()
        .unwrap();
    let mut ret = 0;
    for (i, (a, b)) in a.iter().enumerate() {
        match a.pcmp(&b) {
            PacketOrd::Greater => (),
            _ => ret += i + 1,
        }
    }
    println!("The solution to part1 is: {}", ret);
}
