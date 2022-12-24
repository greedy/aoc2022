use std::{collections::HashMap, convert::TryFrom};
use itertools::Itertools;

use aoc2022::prelude::*;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<22>
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Coord {
    row: usize,
    col: usize
}

enum Cell {
    Open,
    Wall,
}

impl TryFrom<char> for Cell {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Open),
            '#' => Ok(Cell::Wall),
            _ => Err(eyre!("Expect a '.' or '#' only"))
        }
    }
}

#[derive(Default)]
struct Board {
    cells: HashMap<Coord, Cell>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn adjust_coord_torus(&self, coord: &mut Coord, dir: Direction) {
        match dir {
            Direction::Up => {
                if coord.row == 0 {
                    coord.row = self.rows-1;
                } else {
                    coord.row -= 1;
                }
            },
            Direction::Right => {
                coord.col = (coord.col + 1) % self.cols;
            },
            Direction::Down => {
                coord.row = (coord.row + 1) % self.rows;
            },
            Direction::Left => {
                if coord.col == 0 {
                    coord.col = self.cols-1;
                } else {
                    coord.col -= 1;
                }
            }
        }
    }

    fn do_move_torus(&self, coord: &mut Coord, dir: Direction) -> bool {
        let mut probe = coord.clone();
        self.adjust_coord_torus(&mut probe, dir);
        while !self.cells.contains_key(&probe) {
            self.adjust_coord_torus(&mut probe, dir);
        }
        match self.cells.get(&probe).unwrap() {
            Cell::Open => {
                *coord = probe;
                true
            },
            Cell::Wall => false
        }
    }
    
    // part 2
    //
    // My cube is laid out like
    //
    //     aa bb
    //    0AA1BB7
    //    0AA1BB7
    //     22 66
    //    8CC6
    //    8CC6
    //  88 33
    // 0EE4DD7
    // 0EE4DD7
    //  55 99
    // aFF9
    // aFF9
    //  bb
    //
    
    fn adjust_coord_cube(&self, coord: &mut Coord, dir: &mut Direction) {
        match dir {
            Direction::Up => {
                if coord.row == 0 && 50 <= coord.col && coord.col < 100 {
                    let edge_pos = coord.col - 50;
                    // moving from A to F
                    *dir = Direction::Right;
                    coord.col = 0;
                    coord.row = edge_pos + 150;
                } else if coord.row == 0 && 100 <= coord.col && coord.col < 150 {
                    let edge_pos = coord.col - 100;
                    // moving from B to F
                    coord.row = 199;
                    coord.col = edge_pos;
                } else if coord.row == 100 && 0 <= coord.col && coord.col < 50 {
                    let edge_pos = coord.col - 0;
                    // moving from E to C
                    *dir = Direction::Right;
                    coord.col = 50;
                    coord.row = 50 + edge_pos;
                } else {
                    coord.row -= 1;
                }
            },
            Direction::Right => {
                if coord.col == 149 && 0 <= coord.row && coord.row < 50 {
                    let edge_pos = coord.row - 0;
                    // moving from B to D
                    *dir = Direction::Left;
                    coord.col = 99;
                    coord.row = 149 - edge_pos;
                } else if coord.col == 99 && 50 <= coord.row && coord.row < 100 {
                    let edge_pos = coord.row - 50;
                    // moving from C to B
                    *dir = Direction::Up;
                    coord.row = 49;
                    coord.col = 100 + edge_pos;
                } else if coord.col == 99 && 100 <= coord.row && coord.row < 150 {
                    let edge_pos = coord.row - 100;
                    // moving from D to B
                    *dir = Direction::Left;
                    coord.col = 149;
                    coord.row = 49 - edge_pos;
                } else if coord.col == 49 && 150 <= coord.row && coord.row < 200 {
                    let edge_pos = coord.row - 150;
                    // moving from F to D
                    *dir = Direction::Up;
                    coord.row = 149;
                    coord.col = 50 + edge_pos;
                } else {
                    coord.col += 1;
                }
            },
            Direction::Down => {
                if coord.row == 49 && 100 <= coord.col && coord.col < 150 {
                    let edge_pos = coord.col - 100;
                    // moving from B to C
                    *dir = Direction::Left;
                    coord.col = 99;
                    coord.row = edge_pos + 50;
                } else if coord.row == 149 && 50 <= coord.col && coord.col < 100 {
                    let edge_pos = coord.col - 50;
                    // moving from D to F
                    *dir = Direction::Left;
                    coord.col = 49;
                    coord.row = edge_pos + 150;
                } else if coord.row == 199 && 0 <= coord.col && coord.col < 50 {
                    let edge_pos = coord.col - 0;
                    // moving from F to B
                    coord.row = 0;
                    coord.col = edge_pos + 100;
                } else {
                    coord.row += 1;
                }
            },
            Direction::Left => {
                if coord.col == 50 && 0 <= coord.row && coord.row < 50 {
                    let edge_pos = coord.row - 0;
                    // moving from A to E
                    *dir = Direction::Right;
                    coord.col = 0;
                    coord.row = 149 - edge_pos;
                } else if coord.col == 50 && 50 <= coord.row && coord.row < 100 {
                    let edge_pos = coord.row - 50;
                    // moving from C to E
                    *dir = Direction::Down;
                    coord.row = 100;
                    coord.col = 0 + edge_pos;
                } else if coord.col == 0 && 100 <= coord.row && coord.row < 150 {
                    let edge_pos = coord.row - 100;
                    // moving from E to A
                    *dir = Direction::Right;
                    coord.col = 50;
                    coord.row = 49 - edge_pos;
                } else if coord.col == 0 && 150 <= coord.row && coord.row < 200 {
                    let edge_pos = coord.row - 150;
                    // moving from F to A
                    *dir = Direction::Down;
                    coord.row = 0;
                    coord.col = 50 + edge_pos;
                } else {
                    coord.col -= 1;
                }
            },
        }
    }

    fn do_move_cube(&self, coord: &mut Coord, dir: &mut Direction) -> bool {
        let mut dnew = dir.clone();
        let mut probe = coord.clone();
        self.adjust_coord_cube(&mut probe, &mut dnew);
        match self.cells.get(&probe).expect("moved to a defined cell") {
            Cell::Open => {
                *coord = probe;
                *dir = dnew;
                true
            },
            Cell::Wall => false
        }
    }

}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn cw(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn ccw(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn code(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let lines: Vec<_> = cli.input.get_input()?.lines().try_collect()?;

    let (board_part, path_part) = lines.split(String::is_empty).collect_tuple().ok_or_else(|| eyre!("Expected two parts to the input"))?;

    let mut board = Board::default();
    let mut start_coord = None;

    for (row, line) in board_part.iter().enumerate() {
        for (col, b) in line.chars().enumerate().filter(|x| !x.1.is_whitespace()) {
            if start_coord.is_none() {
                start_coord.insert(Coord { row, col });
            }
            let cell = b.try_into()?;
            let coord = Coord { row, col };
            board.cells.insert(coord, cell);
            board.cols = board.cols.max(col+1);
        }
        board.rows = row+1;
    }

    let path = path_part.join("");

    let start_coord = start_coord.expect("a starting position");

    {
        let mut coord = start_coord;
        let mut dir = Direction::Right;

        for step in path.clone().split_inclusive(char::is_alphabetic) {
            let (distance, turn): (usize, Option<&str>) = match step.rfind(char::is_alphabetic) {
                None => (step.parse()?, None),
                Some(i) => {
                    let (d,t) = step.split_at(i);
                    (d.parse()?, Some(t))
                }
            };

            for _ in 0..distance {
                if !board.do_move_torus(&mut coord, dir) {
                    break;
                }
            }

            match turn {
                None => (),
                Some("L") => dir = dir.ccw(),
                Some("R") => dir = dir.cw(),
                Some(_) => bail!("invalid turn")
            }
        }

        let password = 1000 * (coord.row+1) + 4 * (coord.col+1) + dir.code();

        println!("The password is {}", password);
    }

    {
        let mut coord = start_coord;
        let mut dir = Direction::Right;

        for step in path.clone().split_inclusive(char::is_alphabetic) {
            let (distance, turn): (usize, Option<&str>) = match step.rfind(char::is_alphabetic) {
                None => (step.parse()?, None),
                Some(i) => {
                    let (d,t) = step.split_at(i);
                    (d.parse()?, Some(t))
                }
            };

            for _ in 0..distance {
                if !board.do_move_cube(&mut coord, &mut dir) {
                    break;
                }
            }

            match turn {
                None => (),
                Some("L") => dir = dir.ccw(),
                Some("R") => dir = dir.cw(),
                Some(_) => bail!("invalid turn")
            }
        }

        let password = 1000 * (coord.row+1) + 4 * (coord.col+1) + dir.code();

        println!("The password is {}", password);
    }

    Ok(())
}
