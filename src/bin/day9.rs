use aoc2022::prelude::*;
use std::{collections::HashSet, str::FromStr};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<9>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord { x: isize, y: isize }

impl Coord {
    fn adjust(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1
        }
    }

    fn adjusted(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self { y: self.y + 1, ..*self },
            Direction::Down => Self { y: self.y - 1, ..*self },
            Direction::Right => Self { x: self.x + 1, ..*self },
            Direction::Left => Self { x: self.x - 1, ..*self }
        }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl TryFrom<u8> for Direction {
    type Error = Report;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            b'U' => Ok(Self::Up),
            b'D' => Ok(Self::Down),
            b'L' => Ok(Self::Left),
            b'R' => Ok(Self::Right),
            _ => Err(eyre!("{} is not a valid direction", c))
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(eyre!("{} is not a valid direction", c))
        }
    }
}

impl FromStr for Direction {
    type Err = Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(eyre!("{} is not a valid direction", s))
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Motion {
    direction: Direction,
    distance: usize
}

impl Motion {
    pub fn new(direction: Direction, distance: usize) -> Self {
        Self { direction, distance }
    }
}

struct Rope {
    head_pos: Coord,
    tail_pos: Coord,
    tail_image: HashSet<Coord>,
    head_image: HashSet<Coord>,
}

impl Rope {
    pub fn new() -> Self {
        let mut head_image = HashSet::new();
        let mut tail_image = HashSet::new();
        head_image.insert((0, 0).into());
        tail_image.insert((0, 0).into());
        Self {
            head_pos: (0, 0).into(),
            tail_pos: (0, 0).into(),
            head_image,
            tail_image,
        }
    }

    pub fn apply(&mut self, motion: &Motion) {
        for _ in 0..motion.distance {
            self.move_head(motion.direction);
        }
    }

    fn move_head(&mut self, direction: Direction) {
        self.head_pos.adjust(direction);
        debug_assert!(isize::abs(self.tail_pos.y - self.head_pos.y) <= 2);
        debug_assert!(isize::abs(self.tail_pos.x - self.head_pos.x) <= 2);
        debug_assert!(
            (isize::abs(self.tail_pos.y - self.head_pos.y) +
             isize::abs(self.tail_pos.x - self.head_pos.x))
            <= 3);
        if self.head_pos.x == self.tail_pos.x + 2 {
            self.tail_pos.x += 1;
            if self.head_pos.y != self.tail_pos.y {
                debug_assert!(isize::abs(self.tail_pos.y - self.head_pos.y) == 1);
                self.tail_pos.y = self.head_pos.y
            }
        } else if self.head_pos.x == self.tail_pos.x - 2 {
            self.tail_pos.x -= 1;
            if self.head_pos.y != self.tail_pos.y {
                debug_assert!(isize::abs(self.tail_pos.y - self.head_pos.y) == 1);
                self.tail_pos.y = self.head_pos.y
            }
        } else if self.head_pos.y == self.tail_pos.y + 2 {
            self.tail_pos.y += 1;
            if self.head_pos.x != self.tail_pos.x {
                debug_assert!(isize::abs(self.tail_pos.x - self.head_pos.x) == 1);
                self.tail_pos.x = self.head_pos.x;
            }
        } else if self.head_pos.y == self.tail_pos.y - 2 {
            self.tail_pos.y -= 1;
            if self.head_pos.x != self.tail_pos.x {
                debug_assert!(isize::abs(self.tail_pos.x - self.head_pos.x) == 1);
                self.tail_pos.x = self.head_pos.x;
            }
        }
        debug_assert!(isize::abs(self.tail_pos.y - self.head_pos.y) <= 1);
        debug_assert!(isize::abs(self.tail_pos.x - self.head_pos.x) <= 1);
        self.head_image.insert(self.head_pos);
        self.tail_image.insert(self.tail_pos);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut rope = Rope::new();

    for line in cli.input.get_input()?.lines() {
        let line = line?;

        let (dir, amount) = line.split_once(' ').ok_or_else(|| eyre!("Expected a direction and distance separated by space"))?;
        let motion = Motion::new(dir.parse()?, amount.parse()?);

        rope.apply(&motion);
    }

    println!("The rope tail visits {} locations", rope.tail_image.len());

    Ok(())
}
