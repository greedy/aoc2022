use std::io;
use std::io::prelude::*;

use clap::Parser;

use aoc2022::inputs::InputCLI;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<1>
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut maxcals = 0;
    let mut thiscals = 0;

    for line in cli.input.get_input()?.lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.is_empty() {
                if thiscals > maxcals {
                    maxcals = thiscals;
                }
                thiscals = 0;
            } else {
                thiscals += line.parse::<u32>().unwrap();
            }
        }
    }

    println!("Maximum calories is {maxcals}");

    Ok(())
}
