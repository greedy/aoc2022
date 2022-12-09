use std::collections::HashSet;

use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<8>
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Coord {
    row: usize,
    col: usize
}

impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Tree {
    coord: Coord,
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

pub struct Grid(Vec<Vec<Tree>>);

impl Grid {
    pub fn width(&self) -> usize { self.0.len() }
    pub fn height(&self) -> usize { self.0[0].len() }

    pub fn rows(&self) -> Rows { Rows::new(self) }
    pub fn cols(&self) -> Cols { Cols::new(self) }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.row >= 0 && coord.col >= 0 && coord.row < self.height() && coord.col < self.width()
    }

    pub fn view_from(&self, tree: &Tree, direction: Direction) -> ViewFrom {
        ViewFrom::new(self, tree, direction)
    }

    pub fn trees(&self) -> Trees { Trees::new(self) }

    pub fn tree_score(&self, tree: &Tree) -> usize {
        self.view_from(tree, Direction::Up).count()
            * self.view_from(tree, Direction::Down).count()
            * self.view_from(tree, Direction::Left).count()
            * self.view_from(tree, Direction::Right).count()
    }
}

impl<C> std::ops::Index<C> for Grid
where C: Into<Coord>
{
    type Output = Tree;

    fn index(&self, index: C) -> &Self::Output {
        let coord : Coord = index.into();
        &self.0[coord.row][coord.col]
    }
}

pub struct Trees<'a> { grid: &'a Grid, coord: Coord }

impl<'a> Trees<'a> {
    fn new(grid: &'a Grid) -> Self {
        let coord = (0, 0).into();
        Self { grid, coord }
    }
}

impl<'a> Iterator for Trees<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.grid.in_bounds(&self.coord) {
            None
        } else {
            let tree = &self.grid[self.coord];
            self.coord.col += 1;
            if self.coord.col >= self.grid.width() {
                self.coord.col = 0;
                self.coord.row += 1;
            }
            Some(tree)
        }
    }
}

pub struct Rows<'a> { grid: &'a Grid, front: usize, back: usize }

impl<'a> Rows<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid, front: 0, back: grid.height() }
    }
}

impl<'a> Iterator for Rows<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            let row = Row(self.grid, self.front);
            self.front += 1;
            Some(row)
        }
    }
}

impl<'a> DoubleEndedIterator for Rows<'a> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            self.back = self.back - 1;
            let row = Row(self.grid, self.back);
            Some(row)
        }
    }
}

pub struct Row<'a>(&'a Grid, usize);

impl<'a> Row<'a> {
    pub fn iter(&self) -> RowIter<'a, '_> { RowIter::new(self) }

    pub fn grid(&self) -> &'a Grid { self.0 }
    pub fn rownum(&self) -> usize { self.1 }
}

pub struct RowIter<'a, 'r> { row: &'r Row<'a>, front: usize, back: usize }

impl<'a, 'r> RowIter<'a, 'r> {
    fn new(row: &'r Row<'a>) -> Self {
        Self { row, front: 0, back: row.grid().width() }
    }
}

impl<'a> Iterator for RowIter<'a, '_> {
    type Item = &'a Tree;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            let tree = &self.row.grid()[(self.row.rownum(), self.front)];
            self.front += 1;
            Some(tree)
        }
    }
}

impl<'a> DoubleEndedIterator for RowIter<'a, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            self.back -= 1;
            Some(&self.row.grid()[(self.row.rownum(), self.back)])
        }
    }
}

pub struct Cols<'a> { grid: &'a Grid, front: usize, back: usize }

impl<'a> Cols<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self { grid, front: 0, back: grid.width() }
    }
}

impl<'a> Iterator for Cols<'a> {
    type Item = Col<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            let col = Col::new(self.grid, self.front);
            self.front += 1;
            Some(col)
        }
    }
}

impl<'a> DoubleEndedIterator for Cols<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            self.back -= 1;
            Some(Col::new(self.grid, self.back))
        }
    }
}

pub struct Col<'a> { grid: &'a Grid, colnum: usize }

impl<'a> Col<'a> {
    fn new(grid: &'a Grid, colnum: usize) -> Self { Self { grid, colnum } }

    fn iter(&self) -> ColIter<'a, '_> {
        ColIter::new(self)
    }
}

pub struct ColIter<'a, 'c> { col: &'c Col<'a>, front: usize, back: usize }

impl<'a, 'c> ColIter<'a, 'c> {
    fn new(col: &'c Col<'a>) -> Self {
        Self { col, front: 0, back: col.grid.height() }
    }
}

impl<'a> Iterator for ColIter<'a, '_> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            let tree = &self.col.grid[(self.front, self.col.colnum)];
            self.front += 1;
            Some(tree)
        }
    }
}

impl<'a> DoubleEndedIterator for ColIter<'a, '_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            self.back -= 1;
            let tree = &self.col.grid[(self.back, self.col.colnum)];
            Some(tree)
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Coord {
    pub fn advance(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => {
                if self.row == 0 { return false };
                self.row -= 1
            },
            Direction::Down => self.row += 1,
            Direction::Left => {
                if self.col == 0 { return false };
                self.col -= 1
            },
            Direction::Right => self.col += 1
        };
        true
    }
}

pub struct ViewFrom<'a> {
    grid: &'a Grid,
    direction: Direction,
    view_height: i32,
    coord: Coord,
}

impl<'a> ViewFrom<'a> {
    fn new(grid: &'a Grid, tree: &Tree, direction: Direction) -> Self {
        let view_height = tree.height;
        Self { grid, direction, view_height, coord: tree.coord }
    }
}

impl<'a> Iterator for ViewFrom<'a> {
    type Item = &'a Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.grid.in_bounds(&self.coord) { return None }
        if !self.coord.advance(self.direction) { return None }
        if !self.grid.in_bounds(&self.coord) { return None }
        let tree = &self.grid[self.coord];
        if tree.height >= self.view_height {
            self.coord.row = self.grid.height();
            self.coord.col = self.grid.width();
        }
        Some(tree)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let grid = Grid(
        cli.input.get_input()?.lines().enumerate()
        .map(|(row,s)| s.unwrap().chars().enumerate().map(|(col,d)| Tree { coord: Coord { row, col }, height: d.to_digit(10).unwrap() .try_into().unwrap()}).collect())
        .collect());

    let visible = HashSet::new();

    let width = grid.width();

    let visible = grid.rows().fold(visible, |visible,row| row.iter().fold((-1, visible), collect_visible).1);
    let visible = grid.rows().fold(visible, |visible,row| row.iter().rev().fold((-1, visible), collect_visible).1);
    let visible = grid.cols().fold(visible, |visible,col| col.iter().fold((-1, visible), collect_visible).1);
    let visible = grid.cols().fold(visible, |visible,col| col.iter().rev().fold((-1, visible), collect_visible).1);

    println!("{} trees are visible", visible.len());

    let max_score = grid.trees().map(|t| grid.tree_score(t)).max().unwrap();

    println!("The maximum score is {}", max_score);

    Ok(())
}
