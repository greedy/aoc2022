use aoc2022::prelude::*;
use std::{collections::HashSet, str::FromStr, fmt::Display};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<9>
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord { x: isize, y: isize }

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

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

struct Rope<const N: usize> {
    knots: [Coord; N],
    images: [HashSet<Coord>; N],
}

impl<const N:usize> Rope<N> {
    pub fn new() -> Self {
        let knots = [(0,0).into(); N];
        let images = knots.map(|p| HashSet::from([p]));
        Self {
            knots,
            images
        }
    }

    pub fn apply(&mut self, motion: &Motion) {
        for _ in 0..motion.distance {
            self.move_head(motion.direction);
        }
    }

    fn move_head(&mut self, direction: Direction) {
        //println!("Moving {:?}", direction);
        self.knots[0].adjust(direction);
        self.images[0].insert(self.knots[0]);
        //println!("0: {}", self.knots[0]);
        for follower_index in (1..N) {
            let (_, rest) = self.knots.split_at_mut(follower_index - 1);
            let (leader, rest) = rest.split_first_mut().unwrap();
            let (follower, _) = rest.split_first_mut().unwrap();
            debug_assert!(isize::abs(follower.y - leader.y) <= 2, "index={}, follower={}, leader={}", follower_index, follower, leader);
            debug_assert!(isize::abs(follower.x - leader.x) <= 2, "index={}, follower={}, leader={}", follower_index, follower, leader);
            /*
            debug_assert!(
                (isize::abs(follower.y - leader.y) +
                 isize::abs(follower.x - leader.x))
                <= 3,
                "index={}, follower={}, leader={}", follower_index, follower, leader);
            */
            let dx = leader.x - follower.x;
            let dy = leader.y - follower.y;
            match (dx,dy) {
                (0,0) => (),
                (1,0) | (-1,0) | (0,1) | (0,-1) => (),
                (1,1) | (1,-1) | (-1,-1) | (-1,1) => (),
                (2,0) => follower.x += 1,
                (-2,0) => follower.x -= 1,
                (0,2) => follower.y += 1,
                (0,-2) => follower.y -= 1,
                (2,2) | (2,1) | (1,2) => { follower.x += 1; follower.y += 1 },
                (2,-2) | (2,-1) | (1,-2) => { follower.x += 1; follower.y -= 1 },
                (-2,-2) | (-2,-1) | (-1,-2) => { follower.x -= 1; follower.y -= 1 },
                (-2,2) | (-2,1) | (-1,2) => { follower.x -= 1; follower.y += 1 },
                _ => panic!("Impossible (dx,dy) = ({},{})", dx, dy)
            }
            debug_assert!(isize::abs(follower.x - leader.x) <= 1, "After moving, knots not touching! index={}, follower={}, leader={}", follower_index, follower, leader);
            self.images[follower_index].insert(*follower);
            //println!("{}: {}", follower_index, follower);
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut rope2 : Rope<2> = Rope::new();
    let mut rope10 : Rope<10> = Rope::new();

    for line in cli.input.get_input()?.lines() {
        let line = line?;

        let (dir, amount) = line.split_once(' ').ok_or_else(|| eyre!("Expected a direction and distance separated by space"))?;
        let motion = Motion::new(dir.parse()?, amount.parse()?);

        rope2.apply(&motion);
        rope10.apply(&motion);
    }

    println!("Part 1: The rope tail visits {} locations", rope2.images[1].len());
    println!("Part 2: The rope tail visits {} locations", rope10.images[9].len());

    Ok(())
}
