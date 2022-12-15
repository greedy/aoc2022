use std::thread::sleep;
use std::time::Duration;

use aoc2022::prelude::*;
use aoc2022::grid;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<14>
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Air,
    Rock,
    Sand
}

struct Wall(Vec<grid::Coord>);

impl Wall {
    pub fn parse(s: &str) -> nom::IResult<&str, Wall> {
        use nom::sequence::separated_pair;
        use nom::multi::separated_list1;
        use nom::bytes::complete::tag;
        use nom::character::complete::u32;
        use nom::combinator::map;

        let coord_parser =
            map(separated_pair(u32, tag(","), u32),
            |(col, row)| grid::Coord { row: row as usize, col: col as usize });
        map(separated_list1(tag(" -> "), coord_parser),
        |coords| Wall(coords))(s)
    }

    pub fn add_to_grid(&self, grid: &mut grid::Grid<Cell>) {
        for (from, to) in self.0.iter().tuple_windows() {
            if from.row == to.row {
                // horizontal segment
                let row = from.row;
                (from.col.min(to.col)..=to.col.max(from.col))
                    .for_each(|col| grid[&grid::Coord { row, col }] = Cell::Rock)
            } else {
                assert!(from.col == to.col);
                let col = from.col;
                (from.row.min(to.row)..=to.row.max(from.row))
                    .for_each(|row| grid[&grid::Coord { row, col }] = Cell::Rock)
                // vertical segment
            }
        }
    }

    pub fn max_extent(&self) -> grid::Coord {
        max_extent(self.0.iter())
    }
}

fn max_extent<'a, C:std::borrow::Borrow<grid::Coord>, I:IntoIterator<Item=C>>(coords: I) -> grid::Coord {
    coords.into_iter().fold(grid::Coord { row: 0, col: 0 }, |mut extent, coord| {
        let coord = coord.borrow();
        if coord.row > extent.row {
            extent.row = coord.row
        }
        if coord.col > extent.col {
            extent.col = coord.col
        }
        extent
    })
}

impl grid::GridSquareDisplay for Cell {
    fn cell_char(&self) -> char {
        match self {
            Cell::Air => '.',
            Cell::Rock => '#',
            Cell::Sand => 'o',
        }
    }
}

struct SandSimulator {
    grid: grid::Grid<Cell>,
    sand_source: grid::Coord
}

impl SandSimulator {
    fn new(grid: grid::Grid<Cell>) -> Self {
        let sand_source = grid::Coord { row: 0, col: 500 };
        Self { grid, sand_source }
    }
    fn add_sand<'a>(&'a mut self) -> SandFall<'a> {
        SandFall::new(self)
    }
}

struct SandFall<'a> {
    simulator: &'a mut SandSimulator,
    sand_coord: grid::Coord
}

impl<'a> SandFall<'a> {
    fn new(simulator: &'a mut SandSimulator) -> Self {
        let sand_coord = simulator.sand_source;
        Self { simulator, sand_coord }
    }

    fn sand_step(&mut self) -> Result<&grid::Coord, &grid::Coord> {
        if !self.simulator.grid.in_bounds(&self.sand_coord) {
            return Err(&self.sand_coord)
        }
        self.simulator.grid[&self.sand_coord] = Cell::Air;
        if !self.sand_coord.advance(grid::Direction::Down) {
            panic!("Down direction coordinate doesn't exist");
        }
        if !self.simulator.grid.in_bounds(&self.sand_coord) {
            return Err(&self.sand_coord)
        }
        if Cell::Air == self.simulator.grid[&self.sand_coord] {
            self.simulator.grid[&self.sand_coord] = Cell::Sand;
            return Ok(&self.sand_coord)
        }
        if !self.sand_coord.advance(grid::Direction::Left) {
            panic!("Down-left coordinate doesn't exist")
        }
        if Cell::Air == self.simulator.grid[&self.sand_coord] {
            self.simulator.grid[&self.sand_coord] = Cell::Sand;
            return Ok(&self.sand_coord)
        }
        if !self.sand_coord.advance(grid::Direction::Right) {
            panic!("A coordinate that existed before no longer exists")
        }
        if !self.sand_coord.advance(grid::Direction::Right) {
            panic!("Down-right coordinate doesn't exist")
        }
        if Cell::Air == self.simulator.grid[&self.sand_coord] {
            self.simulator.grid[&self.sand_coord] = Cell::Sand;
            return Ok(&self.sand_coord)
        }
        if !self.sand_coord.advance(grid::Direction::Left) {
            panic!("bad move");
        }
        if !self.sand_coord.advance(grid::Direction::Up) {
            panic!("bad move");
        }
        self.simulator.grid[&self.sand_coord] = Cell::Sand;
        Err(&self.sand_coord)
    }

    fn sand_rest(&mut self) -> &grid::Coord {
        while let Ok(_) = self.sand_step() {
        }
        &self.sand_coord
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let walls: Vec<Wall> = cli.input.get_input()?.lines()
        .map(|r| r.map_err(Into::<Report>::into))
            .map(|l| l.and_then(|l| Wall::parse(l.as_str()).map_err(|e| e.to_owned()).map_err(Into::into).map(|p| p.1))).try_collect()?;

    let extent = max_extent(walls.iter().map(Wall::max_extent));

    let mut grid = grid::Grid::fill(extent.col+1, extent.row+1, Cell::Air);

    walls.iter().for_each(|w| w.add_to_grid(&mut grid));

    let mut sim = SandSimulator::new(grid);

    for i in 0.. {
        let final_pos = {
            let mut falling_sand = sim.add_sand();
            falling_sand.sand_rest().clone()
        };

        println!("{}", sim.grid);
        sleep(Duration::from_millis(10));
        if !sim.grid.in_bounds(&final_pos) {
            println!("After {} units, sand falls out the bottom", i);
            break;
        }
    }

    Ok(())
}
