use std::collections::HashSet;

use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<8>
}

#[derive(Hash, PartialEq, Eq)]
struct Tree {
    row: usize,
    col: usize,
    height: i32
}

fn collect_visible<'a>((highest, mut visible): (i32, HashSet<&'a Tree>), tree: &'a Tree) -> (i32, HashSet<&'a Tree>) {
    if tree.height > highest {
        visible.insert(tree);
        (tree.height, visible)
    } else {
        (highest, visible)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let grid : Vec<Vec<Tree>> =
        cli.input.get_input()?.lines().enumerate()
        .map(|(row,s)| s.unwrap().chars().enumerate().map(|(col,d)| Tree { row, col, height: d.to_digit(10).unwrap() .try_into().unwrap()}).collect())
        .collect();

    let _height = grid.len();
    let width = grid[0].len();

    let visible = HashSet::new();

    let visible = grid.iter().fold(visible, |visible,row| row.iter().fold((-1, visible), collect_visible).1);
    let visible = grid.iter().fold(visible, |visible,row| row.iter().rev().fold((-1, visible), collect_visible).1);
    let visible = (0..width).fold(visible, |visible,col| grid.iter().map(|row| &row[col]).fold((-1, visible), collect_visible).1);
    let visible = (0..width).fold(visible, |visible,col| grid.iter().map(|row| &row[col]).rev().fold((-1, visible), collect_visible).1);


    println!("{} trees are visible", visible.len());

    Ok(())
}
