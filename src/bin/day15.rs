use std::collections::HashMap;
use std::collections::HashSet;

use aoc2022::prelude::*;
use aoc2022::grid;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<15>
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn parse(s: &str) -> nom::IResult<&str, Position> {
        use nom::sequence::{preceded, separated_pair};
        use nom::character::complete::i32;
        use nom::bytes::complete::tag;
        use nom::combinator::map;
        map(
            separated_pair(
                preceded(tag("x="), i32),
                tag(", "),
                preceded(tag("y="), i32)
            ),
            |(x, y)| Position { x, y })(s)
    }

    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        let xdist = i32::abs_diff(self.x, other.x);
        let ydist = i32::abs_diff(self.y, other.y);
        xdist + ydist
    }
}

struct SensorReport {
    position: Position,
    closest_beacon: Position
}

impl SensorReport {
    pub fn parse(s: &str) -> nom::IResult<&str, SensorReport> {
        use nom::sequence::{preceded, tuple};
        use nom::bytes::complete::tag;
        use nom::combinator::map;
        map(
            tuple((
                preceded(tag("Sensor at "), Position::parse),
                preceded(tag(": closest beacon is at "), Position::parse)
            )),
            |(position, closest_beacon)| SensorReport { position, closest_beacon })(s)
    }

    fn non_beacon_positions_in_row(&self, row: i32) -> impl Iterator<Item = Position> {
        let beacon_distance = self.position.manhattan_distance(&self.closest_beacon);
        let row_distance = i32::abs_diff(self.position.y, row);
        let col_range =
            if row_distance <= beacon_distance {
                let col_deviation = beacon_distance - row_distance;
                (self.position.x-(col_deviation as i32))..=(self.position.x+(col_deviation as i32))
            } else {
                1..=0
            };
        col_range.map(move |x| Position { x, y: row })
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let reports : Vec<SensorReport> =
        cli.input.get_input()?.lines()
        .map(|l| l.map_err(|e| e.into()))
        .map(|l: Result<String>| l.and_then(|l| SensorReport::parse(l.as_str()).map(|p| p.1).map_err(|e| e.to_owned().into())))
        .try_collect()?;

    let mut common_positions = reports.iter()
        .fold(HashSet::new(), |mut p, r| {p.extend(r.non_beacon_positions_in_row(2000000)); p});

    let common_positions = reports.iter().fold(common_positions, |mut p, r| {p.remove(&r.closest_beacon); p});

    println!("{} positions don't contain beacon", common_positions.len());

    Ok(())
}
