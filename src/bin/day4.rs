use clap::Parser;
use std::io::prelude::*;
use color_eyre::eyre::{Report, Result, eyre};
use std::ops::Range;
use std::str::FromStr;
use itertools::Itertools;

use aoc2022::inputs::InputCLI;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<4>
}

#[derive(Clone, Debug)]
struct Assignment(Range<usize>);

impl Assignment {
    pub fn redundant_with(&self, other: &Assignment) -> bool {
        self.0.start >= other.0.start && self.0.end <= other.0.end
    }
}

impl FromStr for Assignment {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s.split_once('-').ok_or_else(|| eyre!("No '-' in range"))?;
        let start : usize = start.parse()?;
        let end : usize = end.parse()?;
        Ok(Assignment(start..end+1))
    }
}

#[derive(Clone, Debug)]
struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    pub fn has_redundant(&self) -> bool {
        self.0.redundant_with(&self.1) || self.1.redundant_with(&self.0)
    }
}

impl FromStr for AssignmentPair {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        let (first, second) = s.split_once(',').ok_or_else(|| eyre!("No ',' in pair"))?;
        Ok(AssignmentPair(first.parse()?, second.parse()?))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let assignments : Vec<_> =
        cli.input.get_input()?.lines()
        .map(|l| l.map_err(Report::from).and_then(|l| AssignmentPair::from_str(l.as_str())))
        .try_collect()?;

    let redundant_pair_count = assignments.iter()
        .filter(|p| p.has_redundant())
        .count();

    println!("{redundant_pair_count} pairs have a redundant range");

    Ok(())
}
