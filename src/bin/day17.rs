use std::{ops::{Index, IndexMut}, borrow::Borrow, convert::TryFrom, str::FromStr, sync::atomic::{AtomicU32, Ordering}};
use itertools::Itertools;

use aoc2022::prelude::*;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<17>,
    #[arg(long)]
    gui: bool
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Full
}

use Cell::*;

#[derive(Clone, Copy, Debug)]
struct Line([Cell; Line::WIDTH]);

impl Line {
    const WIDTH: usize = 7;

    fn is_clear(&self) -> bool {
        self.0.iter().all(|x| *x == Empty)
    }
}

impl Default for Line {
    fn default() -> Self {
        Self([Empty; 7])
    }
}

impl Index<usize> for Line {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

struct Well {
    lines: Vec<Line>,
}

impl Default for Well {
    fn default() -> Self {
        let lines = vec![Line::default(); 3];
        Self { lines }
    }
}

impl Well {
    fn add_line(&mut self, line: Line) {
        self.lines.push(line)
    }

    fn ensure_height(&mut self, height: usize) {
        if self.lines.len() < height {
            self.lines.resize(height, Line::default());
        }
    }
}

impl Index<&Coord> for Well {
    type Output = Cell;

    fn index(&self, index: &Coord) -> &Self::Output {
        self.lines.get(index.line).map(|line| &line[index.column]).unwrap_or(&Empty)
    }
}

impl IndexMut<&Coord> for Well {
    fn index_mut(&mut self, index: &Coord) -> &mut Self::Output {
        self.ensure_height(index.line + 1);
        &mut self.lines[index.line][index.column]
    }
}

impl Well {
    fn fill<C: Borrow<Coord>>(&mut self, coord: C) {
        self[coord.borrow()] = Full
    }

    fn is_filled<C: Borrow<Coord>>(&self, coord: C) -> bool {
        self[coord.borrow()] == Full
    }

    fn highest_occupied_line(&self) -> Option<usize> {
        let clear_at_top = self.lines.iter().rev().take_while(|l| l.is_clear()).count();
        if self.lines.len() == clear_at_top {
            None
        } else {
            Some(self.lines.len() - clear_at_top)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    HorizLine,
    Cross,
    Ell,
    VertLine,
    Square,
}

use Shape::*;

impl Shape {
    fn sequence() -> impl Iterator<Item = Shape> {
        [HorizLine, Cross, Ell, VertLine, Square].into_iter().cycle()
    }

    fn width(self) -> usize {
        match self {
            HorizLine => 4,
            Cross => 3,
            Ell => 3,
            VertLine => 1,
            Square => 2
        }
    }

    fn height(self) -> usize {
        match self {
            HorizLine => 1,
            Cross => 3,
            Ell => 3,
            VertLine => 4,
            Square => 2,
        }
    }

    fn occupied_coords(self) -> Vec<Coord> {
        match self {
            HorizLine => vec![
                Coord { line: 0, column: 0 },
                Coord { line: 0, column: 1 },
                Coord { line: 0, column: 2 },
                Coord { line: 0, column: 3 },
            ],
            Cross => vec![
                Coord { line: 0, column: 1 },
                Coord { line: 1, column: 0 },
                Coord { line: 1, column: 1 },
                Coord { line: 1, column: 2 },
                Coord { line: 2, column: 1 },
            ],
            Ell => vec![
                Coord { line: 0, column: 0 },
                Coord { line: 0, column: 1 },
                Coord { line: 0, column: 2 },
                Coord { line: 1, column: 2 },
                Coord { line: 2, column: 2 },
            ],
            VertLine => vec![
                Coord { line: 0, column: 0 },
                Coord { line: 1, column: 0 },
                Coord { line: 2, column: 0 },
                Coord { line: 3, column: 0 },
            ],
            Square => vec![
                Coord { line: 0, column: 0 },
                Coord { line: 0, column: 1 },
                Coord { line: 1, column: 0 },
                Coord { line: 1, column: 1 },
            ],
        }
    }
}

struct Positioned<T> {
    position: Coord,
    item: T
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Coord {
    line: usize,
    column: usize
}

impl std::ops::Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord { line: self.line + rhs.line, column: self.column + rhs.column }
    }
}

impl std::ops::Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord { line: self.line + rhs.line, column: self.column + rhs.column }
    }
}

impl std::ops::Add<&Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord { line: self.line + rhs.line, column: self.column + rhs.column }
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord { line: self.line + rhs.line, column: self.column + rhs.column }
    }
}

impl std::ops::AddAssign<&Coord> for Coord {
    fn add_assign(&mut self, rhs: &Coord) {
        self.line += rhs.line;
        self.column += rhs.column;
    }
}

impl<T> std::ops::Add<&Coord> for &Positioned<T>
where T: Clone
{
    type Output = Positioned<T>;

    fn add(self, rhs: &Coord) -> Self::Output {
        Positioned {
            position: &self.position + rhs,
            item: self.item.clone()
        }
    }
}

impl<T> std::ops::Add<&Coord> for Positioned<T>
where T: Clone
{
    type Output = Positioned<T>;

    fn add(self, rhs: &Coord) -> Self::Output {
        Positioned {
            position: &self.position + rhs,
            item: self.item.clone()
        }
    }
}

impl<T> std::ops::AddAssign<&Coord> for &mut Positioned<T>
{
    fn add_assign(&mut self, rhs: &Coord) {
        self.position += rhs
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Down
}

use Direction::*;
use speedy2d::{color::Color, shape::Rectangle};

impl TryFrom<u8> for Direction {
    type Error = Report;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'<' => Ok(Direction::Left),
            b'>' => Ok(Direction::Right),
            b'v' => Ok(Direction::Down),
            _ => bail!("Expected one of '<', '>', or 'v'")
        }
    }
}

impl Coord {
    fn moved(self, direction: Direction) -> Option<Self> {
        match direction {
            Left => {
                if self.column == 0 { None }
                else { Some(Self { column: self.column - 1, line: self.line }) }
            },
            Right => {
                if self.column >= Line::WIDTH - 1 { None }
                else { Some(Self { column: self.column + 1, line: self.line }) }
            },
            Down => {
                if self.line == 0 { None }
                else { Some(Self { column: self.column, line: self.line - 1 }) }
            }
        }
    }

    fn moove(&mut self, direction: Direction) -> bool {
        match direction {
            Left => {
                if self.column == 0 { false }
                else {
                    self.column -= 1;
                    true
                }
            },
            Right => {
                if self.column >= Line::WIDTH-1 { false }
                else { 
                    self.column += 1;
                    true
                }
            },
            Down => {
                if self.line == 0 { false }
                else {
                    self.line -= 1;
                    true
                }
            }
        }
    }
}

impl<T> Positioned<T> {
    fn moved(&self, direction: Direction) -> Option<Self>
        where T: Clone
    {
        self.position.moved(direction).map(|position|
            Self { position, item: self.item.clone() })
    }

    fn moove(&mut self, direction: Direction) -> bool {
        self.position.moove(direction)
    }
}

impl Positioned<Shape> {
    fn occupied_coords<'a>(&'a self) -> impl IntoIterator<Item = Coord> + 'a {
        self.item.occupied_coords().into_iter().map(|coord| coord + self.position)
    }

    fn can_move(&self, direction: Direction, well: &Well) -> bool {
        self.occupied_coords().into_iter().map(|coord| coord.moved(direction))
            .all(|coord| coord.map(|coord| !well.is_filled(coord)).unwrap_or(false))
    }
}

impl Shape {
    fn at(self, position: Coord) -> Positioned<Shape> {
        Positioned { item: self, position }
    }
}

impl Well {
    fn add(&mut self, rock: &Positioned<Shape>) {
        for coord in rock.occupied_coords() {
            debug_assert!(!self.is_filled(coord));
            self.fill(coord)
        }
    }
}

struct Main {
    simulator: Simulator,
    cell_size: u32,
    tick_rate: std::sync::Arc<AtomicU32>,
    win_size: speedy2d::dimen::UVec2,
}

impl Main {
    fn new(jets: Vec<Direction>) -> Self {
        Self {
            simulator: Simulator::new(jets),
            cell_size: 16,
            tick_rate: std::sync::Arc::new(1000.into()),
            win_size: speedy2d::dimen::UVec2::ZERO,
        }
    }

    fn fill_cell(&self, coord: &Coord, color: Color, graphics: &mut speedy2d::Graphics2D) {
        let well_width = self.cell_size * (Line::WIDTH as u32 + 2);
        let well_x = ((self.win_size.x - well_width) / 2) as f32;
        let well_width = well_width as f32;
        let well_height = self.win_size.y as f32 - 8.0;

        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (well_x + (self.cell_size * (coord.column as u32 + 1)) as f32 + 2.5, well_height + 4.5 - (self.cell_size * (coord.line as u32 + 1)) as f32),
                (well_x + (self.cell_size * (coord.column as u32 + 2)) as f32 - 2.5, well_height + 4.5 - (self.cell_size * (coord.line as u32 + 2)) as f32 - 2.0)), color);
    }
}

impl speedy2d::window::WindowHandler for Main {
    fn on_start(&mut self, helper: &mut speedy2d::window::WindowHelper<()>, info: speedy2d::window::WindowStartupInfo) {
        self.win_size = *info.viewport_size_pixels();

        let eventer = helper.create_user_event_sender();
        let tick_rate = self.tick_rate.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1) / tick_rate.load(Ordering::Relaxed));
                eventer.send_event(()).unwrap()
            }
        });
    }

    fn on_resize(&mut self, helper: &mut speedy2d::window::WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2) {
        self.win_size = size_pixels;
        helper.request_redraw();
    }

    fn on_user_event(&mut self, helper: &mut speedy2d::window::WindowHelper<()>, _user_event: ()) {
        self.simulator.step();
        if self.simulator.falling_rock.is_none() && self.simulator.rock_count == 2022 {
            println!("After 2022 rocks, the tower height is {}", self.simulator.well.highest_occupied_line().unwrap());
            helper.terminate_loop()
        } else {
            helper.request_redraw();
        }
    }

    fn on_draw(&mut self, _helper: &mut speedy2d::window::WindowHelper<()>, graphics: &mut speedy2d::Graphics2D) {
        let well_width = self.cell_size * (Line::WIDTH as u32 + 2);
        let well_x = ((self.win_size.x - well_width) / 2) as f32;
        let well_width = well_width as f32;
        let well_height = self.win_size.y as f32 - 8.0;

        graphics.clear_screen(Color::WHITE);

        graphics.draw_line((well_x + 0.5, 4.5), (well_x + 0.5, well_height + 4.5), 1.0, Color::BLACK);
        graphics.draw_line((well_x + well_width + 0.5, 4.5), (well_x + well_width + 0.5, well_height + 4.5), 1.0, Color::BLACK);
        graphics.draw_line((well_x + 0.5, well_height + 4.5), (well_x + well_width + 0.5, well_height + 4.5), 1.0, Color::BLACK);

        for (line_num, line) in self.simulator.well.lines.iter().enumerate() {
            for (col_num, cell) in line.0.iter().enumerate() {
                if *cell == Full {
                    self.fill_cell(&Coord { line: line_num, column: col_num }, Color::DARK_GRAY, graphics);
                }
            }
        }

        self.simulator.falling_rock.iter().for_each(|(_, rock)| {
            rock.occupied_coords().into_iter().for_each(|coord| self.fill_cell(&coord, Color::LIGHT_GRAY, graphics));
        })
    }

}

struct Simulator {
    well: Well,
    jets: Box<dyn Iterator<Item = Direction>>,
    shapes: Box<dyn Iterator<Item = Shape>>,
    falling_rock: Option<(usize, Positioned<Shape>)>,
    rock_count: u32,
}

impl Simulator {
    fn new(jets: Vec<Direction>) -> Simulator {
        Self {
            jets: std::boxed::Box::new(jets.into_iter().cycle()),
            well: Well::default(),
            shapes: std::boxed::Box::new(Shape::sequence()),
            falling_rock: None,
            rock_count: 0
        }
    }

    fn step(&mut self) {
        let (jet_pos, rock) = self.falling_rock.get_or_insert_with(|| {
            let starting_position = Coord {
                line: self.well.highest_occupied_line().unwrap_or(0) + 3,
                column: 2
            };
            self.rock_count += 1;
            (0, self.shapes.next().unwrap().at(starting_position))
        });
        let jet_dir = self.jets.next().unwrap();
        if rock.can_move(jet_dir, &self.well) {
            rock.moove(jet_dir);
        }
        if rock.can_move(Direction::Down, &self.well) {
            rock.moove(Direction::Down);
        } else {
            self.well.add(&rock);
            self.falling_rock = None;
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let jets = cli.input.get_input()?.bytes().take_while(|b| b.iter().all(|b| !b.is_ascii_whitespace())).into_eyre().map_and_then(Direction::try_from).try_collect()?;


    if (cli.gui) {
        let window = speedy2d::Window::new_centered("Christmas Tree Rocktris", (16 * 25, 800)).unwrap();
        window.run_loop(Main::new(jets));
    } else {
        let mut sim = Simulator::new(jets);
        while sim.falling_rock.is_some() || sim.rock_count < 2022 {
            sim.step();
        }

        println!("After 2022 rocks, the tower height is {}", sim.well.highest_occupied_line().unwrap());
    }

    Ok(())
}
