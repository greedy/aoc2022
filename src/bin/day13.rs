use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<13>
}

#[derive(Clone)]
enum Packet {
    Int(i32),
    List(Vec<Packet>)
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => write!(f, "{}", n),
            Self::List(v) => {
                write!(f, "[")?;
                if let Some((first, rest)) = v.split_first() {
                    write!(f, "{}", first)?;
                    rest.iter().map(|p| write!(f, ",{}", p)).try_collect()?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}

impl Packet {
    pub fn parse(s: &str) -> nom::IResult<&str, Self> {
        use nom::combinator::map;
        use nom::branch::alt;
        use nom::sequence::delimited;
        use nom::multi::separated_list0;
        use nom::bytes::complete::tag;
        use nom::character::complete::i32;

        let num_parser = map(i32, (|n| Self::Int(n)));
        let list_parser = map(delimited(tag("["), separated_list0(tag(","), Self::parse), tag("]")), |v| Self::List(v));

        alt((num_parser, list_parser))(s)
    }
}

impl PartialEq<Packet> for Packet {
    fn eq(&self, other: &Packet) -> bool {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (a@Self::Int(_), Self::List(b)) => b.len() == 1 && b[0] == *a,
            (Self::List(a), b@Self::Int(_)) => a.len() == 1 && a[0] == *b
        }
    }
}

impl Eq for Packet { }

impl PartialOrd<Packet> for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => i32::cmp(a, b),
            (Self::Int(a), b@Self::List(_)) => Self::List(vec![Self::Int(*a)]).cmp(b),
            (a@Self::List(_), Self::Int(b)) => a.cmp(&Self::List(vec![Self::Int(*b)])),
            (Self::List(a), Self::List(b)) => {
                let mut a_iter = a.iter();
                let mut b_iter = b.iter();
                loop {
                    match (a_iter.next(), b_iter.next()) {
                        (Some(_), None) => break std::cmp::Ordering::Greater,
                        (None, Some(_)) => break std::cmp::Ordering::Less,
                        (None, None) => break std::cmp::Ordering::Equal,
                        (Some(x), Some(y)) => {
                            match x.cmp(y) {
                                std::cmp::Ordering::Equal => continue,
                                ord => break ord
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut lines_iter = cli.input.get_input()?.lines();

    let mut part1_answer = 0;
    let mut index = 1;

    let mut all_packets: Vec<Packet> = Vec::new();

    while let Some(first) = lines_iter.next() {
        let first = first?;
        let second = lines_iter.next().ok_or_else(|| eyre!("Missing second packet in pair"))??;

        let first = Packet::parse(first.as_str()).map_err(|e| e.to_owned())?.1;
        let second = Packet::parse(second.as_str()).map_err(|e| e.to_owned())?.1;

        if first < second {
            part1_answer += index;
        }

        all_packets.push(first);
        all_packets.push(second);

        index += 1;

        match lines_iter.next() {
            None => break,
            Some(Err(e)) => return Err(e.into()),
            Some(Ok(l)) => {
                if !l.is_empty() {
                    bail!("Expected blank line but was {:?}", l)
                }
            }
        }
    }

    println!("Part 1 answer: {}", part1_answer);

    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider_6 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    all_packets.push(divider_2.clone());
    all_packets.push(divider_6.clone());

    all_packets.sort();

    let index_2 = 1 + all_packets.binary_search(&divider_2).map_err(|_| eyre!("Divider [[2]] not found"))?;
    let index_6 = 1 + all_packets.binary_search(&divider_6).map_err(|_| eyre!("Divider [[6]] not found"))?;

    dbg!(index_2);
    dbg!(index_6);

    println!("Part 2 answer: {}", index_2 * index_6);

    Ok(())
}
