use std::io;
use std::io::prelude::*;

use clap::Parser;

use aoc2022::inputs::InputCLI;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<1>
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut calories = Vec::new();
    let mut thiscals = 0;

    for line in cli.input.get_input()?.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.is_empty() {
                calories.push(thiscals);
                thiscals = 0;
            } else {
                thiscals += line.parse::<u32>().unwrap();
            }
        }
    }

    calories.sort();

    let maxcals = calories.last().unwrap();
    println!("Maximum calories is {maxcals}");

    let top3 : u32 = (calories[calories.len()-3..].iter()).sum();

    println!("Combined calories of top3 is {top3}");

    Ok(())
}
