use aoc2022::prelude::*;

mod astar;
mod grid;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<12>
}

#[derive(Copy, Clone, Debug)]
struct Square {
    elevation: u8
}

impl Square {
    fn from_byte(b: u8) -> Option<Self> {
        if b.is_ascii_lowercase() {
            Some(Self { elevation: b - b'a' })
        } else {
            None
        }
    }

    fn can_climb_to(&self, other: Self) -> bool {
        other.elevation <= self.elevation + 1
    }
}

struct Problem {
    grid: grid::Grid<Square>,
    start_pos: grid::Coord,
    end_pos: grid::Coord
}

impl<'a> astar::SearchProblem<'a> for Problem {
    type Node = grid::Coord;
    type SuccessorIter = SuccIter<'a>;

    fn start(&self) -> Self::Node {
        self.start_pos
    }

    fn goal(&self) -> Self::Node {
        self.end_pos
    }

    fn distance_heuristic(&self, n: &Self::Node) -> usize {
        let horiz_distance = n.manhattan_distance(&self.goal());
        let vert_distance = self.grid[&self.goal()].elevation - self.grid[n].elevation;
        usize::max(horiz_distance, vert_distance.into())
    }

    fn successors(&'a self, n: Self::Node) -> SuccIter<'a> {
        SuccIter::new(&self.grid, n)
    }
}

struct SuccIter<'a> {
    grid: &'a grid::Grid<Square>,
    curr: grid::Coord,
    i: u8
}

impl<'a> SuccIter<'a> {
    fn new(grid: &'a grid::Grid<Square>, curr: grid::Coord) -> Self {
        Self { grid, curr, i: 0 }
    }
}

impl Iterator for SuccIter<'_> {
    type Item = grid::Coord;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let dir = match self.i {
                0 => grid::Direction::Up,
                1 => grid::Direction::Down,
                2 => grid::Direction::Left,
                3 => grid::Direction::Right,
                _ => return None
            };
            self.i += 1;
            let next_coord =
                if let Some(c) = self.curr.advanced(dir) {
                    c
                } else {
                    continue;
                };
            let next_square =
                if let Some(sq) = self.grid.get(&next_coord) {
                    sq
                } else {
                    continue;
                };
            let cur_square = self.grid.get(&self.curr).unwrap();
            if cur_square.can_climb_to(*next_square) {
                return Some(next_coord)
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut grid : grid::Grid<Square> = grid::Grid::empty();

    let mut start_pos = grid::Coord { row: 0, col: 0 };
    let mut end_pos = grid::Coord { row: 0, col: 0 };

    for (row, line) in cli.input.get_input()?.lines().enumerate() {
        let line = line?;

        let row : Vec<_> = line.bytes().enumerate()
            .map(|(col, b)| {
                if b == b'S' {
                    start_pos = grid::Coord { row, col };
                    b'a'
                } else if b == b'E' {
                    end_pos = grid::Coord { row, col };
                    b'z'
                } else {
                    b
                }
            })
            .map(|b| Square::from_byte(b).unwrap())
                .collect();

        grid.push_row(row).map_err(|_| eyre!("Couldn't push row"))?;
    }

    println!("Grid size is {}x{}", grid.width(), grid.height());

    let mut problem = Problem { grid, start_pos, end_pos };

    let cost = astar::astar(&problem).ok_or_else(|| eyre!("No path from start to end!"))?;

    println!("Shortest path has length {}", cost);

    let mut grid = problem.grid;

    let bestest = grid.rows().enumerate().flat_map(|(row, r)| r.iter().enumerate().map(move |(col, sq)| (grid::Coord { row, col }, sq)))
        .filter_map(|(coord, sq)| {
            if sq.elevation != 0 {
                None
            } else {
                let p = Problem { grid: grid.clone(), start_pos: coord, end_pos };
                astar::astar(&p)
            }
        }).min().unwrap();

    println!("shortest hike is length {}", bestest);

    Ok(())
}
