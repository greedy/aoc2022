use std::ops::{Index, IndexMut};
use std::fmt::{Display, Write};

use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Coord {
    pub row: usize,
    pub col: usize
}

impl Coord {
    pub fn manhattan_distance(&self, other: &Coord) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    items: Vec<T>
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn rows(&self) -> Rows<T> { Rows::new(self) }
    pub fn cols(&self) -> Cols<T> { Cols::new(self) }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.row < self.height() && coord.col < self.width()
    }

    pub fn get(&self, coord: &Coord) -> Option<&T> {
        if self.in_bounds(coord) {
            Some(&self[coord])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut T> {
        if self.in_bounds(coord) {
            Some(&mut self[coord])
        } else {
            None
        }
    }

    pub fn get_row(&self, row: usize) -> Option<Row<T>> {
        if row >= self.height() { return None }
        Some(Row::new(self, row))
    }

    pub fn get_col(&self, col: usize) -> Option<Col<T>> {
        if col >= self.width() { return None }
        Some(Col::new(self, col))
    }

    pub fn fill(width: usize, height: usize, value: T) -> Self where T: Clone {
        Self { width, height, items: vec![value; width * height] }
    }

    pub fn empty() -> Self {
        Self { width: 0, height: 0, items: vec![] }
    }

    pub fn push_row(&mut self, row: Vec<T>) -> Result<(), Vec<T>> {
        if self.width == 0 && self.height == 0 {
            self.width = row.len();
            self.height = 1;
            self.items = row;
            return Ok(())
        }

        if row.len() != self.width {
            return Err(row)
        }

        self.height += 1;
        let mut row = row;
        self.items.append(&mut row);
        Ok(())
    }
}

impl<T> Index<&Coord> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Coord) -> &Self::Output {
        assert!(self.in_bounds(index));
        &self.items[index.row * self.width + index.col]
    }
}

impl<T> IndexMut<&Coord> for Grid<T> {

    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        assert!(self.in_bounds(index));
        &mut self.items[index.row * self.width + index.col]
    }
}

pub struct Rows<'a, T> { grid: &'a Grid<T>, front: usize, back: usize }

impl<'a, T> Rows<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, front: 0, back: grid.height() }
    }

    fn done(&self) -> bool {
        self.front >= self.back
    }
}

impl<'a, T> Iterator for Rows<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            let row = Row::new(self.grid, self.front);
            self.front += 1;
            Some(row)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let d = self.back - self.front;
        (d, Some(d))
    }

    fn count(self) -> usize {
        self.back - self.front
    }

    fn last(self) -> Option<Self::Item> {
        if self.done() { return None }
        Some(Row::new(self.grid, self.back - 1))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n >= self.len() { return None }
        self.front += n + 1;
        Some(Row::new(self.grid, self.front - 1))
    }
}

impl<'a, T> ExactSizeIterator for Rows<'a, T> { }

impl<'a, T> DoubleEndedIterator for Rows<'a, T> {

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            self.back = self.back - 1;
            let row = Row::new(self.grid, self.back);
            Some(row)
        }
    }
}

pub struct Row<'a, T> {
    grid: &'a Grid<T>,
    row: usize
}

impl<'a, T> Row<'a, T> {
    fn new(grid: &'a Grid<T>, row: usize) -> Self {
        Self { grid, row }
    }

    pub fn iter(&self) -> RowIter<'a, T> { RowIter::new(self) }

    fn grid(&self) -> &'a Grid<T> { self.grid }
    fn rownum(&self) -> usize { self.row }

    fn mk_coord(&self, col: usize) -> Coord {
        Coord { row: self.row, col }
    }

    pub fn get(&self, col: usize) -> Option<&'a T> {
        self.grid.get(&self.mk_coord(col))
    }
}

impl<'a, T> Index<usize> for Row<'a, T> {
    type Output = T;

    fn index(&self, col: usize) -> &Self::Output {
        &self.grid[&self.mk_coord(col)]
    }
}

pub struct RowIter<'a, T> {
    grid: &'a Grid<T>,
    front: Coord,
    back: Coord
}

impl<'a, T> RowIter<'a, T> {
    fn new(r: &Row<'a, T>) -> Self {
        let grid = r.grid;
        let row = r.row;
        let front = Coord { row: row, col: 0 };
        let back = Coord { row: row, col: grid.width() };
        Self { grid, front, back }
    }
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front.col >= self.back.col {
            None
        } else {
            let item = &self.grid[&self.front];
            self.front.advance(Direction::Right);
            Some(item)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let d = self.front.manhattan_distance(&self.back);
        (d, Some(d))
    }
}

impl<'a, T> ExactSizeIterator for RowIter<'a, T> {}

impl<'a, T> DoubleEndedIterator for RowIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front.col >= self.back.col {
            None
        } else {
            self.back.advance(Direction::Left);
            Some(&self.grid[&self.back])
        }
    }
}

pub struct Cols<'a, T> { grid: &'a Grid<T>, front: usize, back: usize }

impl<'a, T> Cols<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, front: 0, back: grid.width() }
    }
}

impl<'a, T> Iterator for Cols<'a, T> {
    type Item = Col<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            let col = Col::new(self.grid, self.front);
            self.front += 1;
            Some(col)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let d = self.back - self.front;
        (d, Some(d))
    }
}

impl<'a, T> ExactSizeIterator for Cols<'a, T> { }

impl<'a, T> DoubleEndedIterator for Cols<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            None
        } else {
            self.back -= 1;
            Some(Col::new(self.grid, self.back))
        }
    }
}

pub struct Col<'a, T> { grid: &'a Grid<T>, colnum: usize }

impl<'a, T> Col<'a, T> {
    fn new(grid: &'a Grid<T>, colnum: usize) -> Self { Self { grid, colnum } }

    pub fn iter(&self) -> ColIter<'a, T> {
        ColIter::new(self)
    }

    fn mk_coord(&self, row: usize) -> Coord {
        Coord { row, col: self.colnum }
    }
}

pub struct ColIter<'a, T> { grid: &'a Grid<T>, front: Coord, back: Coord }

impl<'a, T> ColIter<'a, T> {
    fn new(c: &Col<'a, T>) -> Self {
        let grid = c.grid;
        let front = c.mk_coord(0);
        let back = c.mk_coord(grid.height);
        Self { grid, front, back }
    }

    fn done(&self) -> bool {
        self.front.row >= self.back.row
    }
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done() {
            None
        } else {
            let tree = &self.grid[&self.front];
            self.front.advance(Direction::Down);
            Some(tree)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let d = self.front.manhattan_distance(&self.back);
        (d, Some(d))
    }
}

impl<'a, T> ExactSizeIterator for ColIter<'a, T> {}

impl<'a, T> DoubleEndedIterator for ColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.done() {
            None
        } else {
            self.back.advance(Direction::Up);
            let tree = &self.grid[&self.back];
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

impl Direction {
    pub fn all() -> impl Iterator<Item = Self> {
        std::iter::once(Self::Up)
            .chain(std::iter::once(Self::Down))
            .chain(std::iter::once(Self::Left))
            .chain(std::iter::once(Self::Right))
    }
}

impl Coord {

    pub fn advance(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => {
                let (row_up, overflow) = self.row.overflowing_sub(1);
                if overflow { return false };
                self.row = row_up;
                true
            },
            Direction::Down => {
                let (row_down, overflow) = self.row.overflowing_add(1);
                if overflow { return false };
                self.row = row_down;
                true
            },
            Direction::Left => {
                let (col_up, overflow) = self.col.overflowing_sub(1);
                if overflow { return false };
                self.col = col_up;
                true
            },
            Direction::Right => {
                let (col_down, overflow) = self.col.overflowing_add(1);
                if overflow { return false };
                self.col = col_down;
                true
            }
        }
    }

    pub fn try_into_advanced(mut self, dir: Direction) -> Result<Self, Self> {
        if self.advance(dir) {
            Ok(self)
        } else {
            Err(self)
        }
    }

    pub fn advanced(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => {
                self.row.checked_sub(1).map(|row| Self { row, ..*self })
            }
            Direction::Down => {
                self.row.checked_add(1).map(|row| Self { row, ..*self })
            }
            Direction::Left => {
                self.col.checked_sub(1).map(|col| Self { col, ..*self })
            }
            Direction::Right => {
                self.col.checked_add(1).map(|col| Self { col, ..*self })
            }
        }
    }
}

pub trait GridSquareDisplay {
    fn cell_char(&self) -> char;
}

impl<T:GridSquareDisplay> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.rows().map(|r| {
            r.iter().map(|c| f.write_char(c.cell_char())).try_collect()?;
            f.write_char('\n')
            }).try_collect()
    }
}

