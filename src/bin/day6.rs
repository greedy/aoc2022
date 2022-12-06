use aoc2022::prelude::*;
use itertools::{process_results, Itertools};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<6>
}

struct Windows<'a, T>{
    inner: &'a [T],
    window_size: usize,
    position: usize
}

impl<'a, T> Windows<'a, T> {
    pub fn new(inner: &'a [T], window_size: usize) -> Self {
        Windows { inner, window_size, position: 0 }
    }
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.position > self.inner.len() { return None; }
        let answer = self.inner.get(self.position..(self.position + self.window_size));
        self.position += self.window_size;
        answer
    }
}

trait WindowsIterExt<T> {
    fn windows(&self, n: usize) -> Windows<T>;
}

impl<T> WindowsIterExt<T> for [T] {
    fn windows(&self, n: usize) -> Windows<T> {
        Windows::new(self, n)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let input : Vec<_> = cli.input.get_input()?.bytes().try_collect()?;

    if let Some(position) = input.windows(4).position(|w| w.iter().all_unique()) {
        println!("Part 1: Start marker is at position {}", position + 4);
    } else {
        println!("Part 1: No start marker found!");
    }

    if let Some(position) = input.windows(14).position(|w| w.iter().all_unique()) {
        println!("Part 2: Start marker is at position {}", position + 14);
    } else {
        println!("Part 2: No start marker found!");
    }

    Ok(())
}
