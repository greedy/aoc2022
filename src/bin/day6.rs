use aoc2022::prelude::*;
use itertools::{process_results, Itertools};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<6>
}

fn all_distinct<T:PartialEq>(a: T, b: T, c: T, d: T) -> bool {
    if a == b { return false; }
    if a == c { return false; }
    if a == d { return false; }
    if b == c { return false; }
    if b == d { return false; }
    if c == d { return false; }
    true
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    if let Some(position) = process_results(
        cli.input.get_input()?.bytes(),
        |it| it.map_into::<char>().tuple_windows().inspect(|t| println!("Checking window {:?}", t)).position(|(a,b,c,d)| all_distinct(a,b,c,d)))? {
        println!("Start marker is at position {}", position + 4);
    } else {
        println!("No start marker found!");
    }

    Ok(())
}
