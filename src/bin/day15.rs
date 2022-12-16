use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeBounds;
use std::ops::RangeInclusive;

use aoc2022::prelude::*;
use aoc2022::grid;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<15>
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32
}

fn merge_inclusive_ranges<T:Ord + Clone>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> RangeInclusive<T> {
    RangeInclusive::new(a.start().min(b.start()).clone(), a.end().max(b.end()).clone())
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

    fn distance_to_beacon(&self) -> u32 {
        self.position.manhattan_distance(&self.closest_beacon)
    }

    fn non_beacon_positions_in_row(&self, row: i32) -> impl Iterator<Item = Position> {
        let col_range = self.non_beacon_range_in_row(row);
        col_range.map(move |x| Position { x, y: row })
    }

    fn non_beacon_range_in_row(&self, row: i32) -> RangeInclusive<i32> {
        let beacon_distance = self.position.manhattan_distance(&self.closest_beacon);
        let row_distance = i32::abs_diff(self.position.y, row);
        if row_distance <= beacon_distance {
            let col_deviation = beacon_distance - row_distance;
            (self.position.x-(col_deviation as i32))..=(self.position.x+(col_deviation as i32))
        } else {
            1..=0
        }
    }
}

#[derive(Debug)]
struct IntervalSet<T> {
    intervals: Vec<RangeInclusive<T>>
}

impl<T:Ord + Clone> IntervalSet<T> {
    fn new() -> Self {
        Self { intervals: Vec::new() }
    }

    fn add(&mut self, interval: RangeInclusive<T>) {
        if self.intervals.is_empty() {
            self.intervals.push(interval)
        } else {
            let left_edge = self.intervals.partition_point(|existing| existing.end() < interval.start());
            // intervals[left_edge-1].end < interval.start
            let right_edge = self.intervals.partition_point(|existing| existing.start() <= interval.end());
            // intervals[right_edge].start() > interval.end
            // intervals[left_edge..right_edge] will be replaced
            let replaced_range = left_edge..right_edge;
            if replaced_range.is_empty() {
                // just insert the interval
                self.intervals.insert(left_edge, interval);
            } else if replaced_range.len() == 1 {
                let previous = &self.intervals[left_edge];
                if previous.contains(interval.start()) && previous.contains(interval.end()) {
                    // interval already in the set
                    return;
                } else {
                    // need to replace previous by something bigger
                    let replacement = RangeInclusive::new(previous.start().min(interval.start()).clone(), previous.end().max(interval.end()).clone());
                    self.intervals[left_edge] = replacement;
                }
            } else {
                let first = &self.intervals[left_edge];
                let last = &self.intervals[right_edge-1];
                let replacement =
                    if interval.contains(first.start()) && interval.contains(last.end()) {
                        interval
                    } else {
                        let start = first.start().min(interval.start()).clone();
                        let end = last.end().max(interval.end()).clone();
                        start..=end
                    };
                self.intervals.splice(replaced_range, std::iter::once(replacement));
            }
        }
    }

    fn contains(&self, point: &T) -> bool {
        if let Some(candidate) = self.intervals.get(self.intervals.partition_point(|i| i.start() < point)) {
            candidate.contains(point)
        } else {
            false
        }
    }

    fn contains_range(&self, range: &RangeInclusive<T>) -> bool {
        if let Some(candidate) = self.intervals.get(self.intervals.partition_point(|i| i.start() < range.start())) {
            candidate.contains(range.start()) && candidate.contains(range.end())
        } else {
            false
        }
    }

    fn gaps<'a>(&'a self) -> impl Iterator<Item=RangeInclusive<T>> + 'a
        where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T>,
              i32: Into<T>
    {
        self.intervals.iter()
            .tuple_windows()
            .map(|(a,b)| RangeInclusive::new(a.end().clone()+1.into(), b.start().clone()-1.into()))
            .filter(|g| !g.is_empty())
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

    let mut common_positions : HashSet<Position> = reports.iter()
        .flat_map(|r| r.non_beacon_positions_in_row(2000000))
        .collect();

    reports.iter().for_each(|r| { common_positions.remove(&r.closest_beacon); });

    println!("{} positions don't contain beacon", common_positions.len());

    for row in (0..=4000000).rev() {
        let mut non_beacon_cols = IntervalSet::new();
        reports.iter()
            .map(|report| report.non_beacon_range_in_row(row))
            .for_each(|range| non_beacon_cols.add(range));
        let mut gaps = non_beacon_cols.gaps()
            .filter(|r| r.start() >= &0 && r.end() <= &4000000);
        if let Some(gap) = gaps.next()
        {
            let col = *gap.start() as usize;
            println!("Distress beacon is at x={}, y={}", col, row);
            let freq = col * 4000000 + row as usize;
            println!("Frequencey is {}", freq);
            break;
        }
    }

    Ok(())
}
