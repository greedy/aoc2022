use clap::Parser;
use std::io;
use std::io::prelude::*;
use parse_display::{Display, FromStr};

use aoc2022::inputs::InputCLI;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<2>
}

enum Player {
    Us,
    Them
}

#[derive(Debug, Display, FromStr)]
enum OpponentPlay {
    #[display("A")]
    Rock,
    #[display("B")]
    Paper,
    #[display("C")]
    Scissors
}

#[derive(Debug, Display, FromStr)]
enum OurPlay {
    #[display("X")]
    Rock,
    #[display("Y")]
    Paper,
    #[display("Z")]
    Scissors
}

impl OurPlay {
    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }
}

#[derive(Debug, Display, FromStr)]
#[display("{them} {us}")]
struct Round {
    them: OpponentPlay,
    us: OurPlay
}

#[derive(Debug, Display, FromStr)]
enum Outcome {
    #[display("Z")]
    Win,
    #[display("X")]
    Lose,
    #[display("Y")]
    Draw
}

#[derive(Debug, Display, FromStr)]
#[display("{them} {result}")]
struct RoundPartTwo {
    them: OpponentPlay,
    result: Outcome
}

impl RoundPartTwo {
    pub fn our_play(&self) -> OurPlay {
        match (&self.them, &self.result) {
            (OpponentPlay::Scissors, Outcome::Win) => OurPlay::Rock,
            (OpponentPlay::Scissors, Outcome::Lose) => OurPlay::Paper,
            (OpponentPlay::Scissors, Outcome::Draw) => OurPlay::Scissors,
            (OpponentPlay::Rock, Outcome::Win) => OurPlay::Paper,
            (OpponentPlay::Rock, Outcome::Lose) => OurPlay::Scissors,
            (OpponentPlay::Rock, Outcome::Draw) => OurPlay::Rock,
            (OpponentPlay::Paper, Outcome::Win) => OurPlay::Scissors,
            (OpponentPlay::Paper, Outcome::Lose) => OurPlay::Rock,
            (OpponentPlay::Paper, Outcome::Draw) => OurPlay::Paper,
        }
    }

    pub fn score(&self) -> u32 {
        self.result.score() + self.our_play().score()
    }
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3
        }
    }
}

impl Round {
    pub fn outcome(&self) -> Outcome {
        match self.them {
            OpponentPlay::Rock =>
                match self.us {
                    OurPlay::Rock => Outcome::Draw,
                    OurPlay::Paper => Outcome::Win,
                    OurPlay::Scissors => Outcome::Lose,
                },
            OpponentPlay::Paper =>
                match self.us {
                    OurPlay::Rock => Outcome::Lose,
                    OurPlay::Paper => Outcome::Draw,
                    OurPlay::Scissors => Outcome::Win
                },
            OpponentPlay::Scissors =>
                match self.us {
                    OurPlay::Paper => Outcome::Lose,
                    OurPlay::Scissors => Outcome::Draw,
                    OurPlay::Rock => Outcome::Win
                }
        }
    }

    pub fn score(&self) -> u32 {
        self.outcome().score() + self.us.score()
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut total_score = 0;
    let mut part2_score = 0;

    for line in cli.input.get_input()?.lines() {
        let line = line.unwrap();
        let trimmed = line.trim();
        let round : Round = trimmed.parse().unwrap();
        let round2 : RoundPartTwo = trimmed.parse().unwrap();
        total_score += round.score();
        part2_score += round2.score();
    }

    println!("Total score: {total_score}");
    println!("Part 2 score: {part2_score}");

    Ok(())
}
