use aoc2022::prelude::*;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<24>
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    pub fn north(&self) -> Coord {
        Self { x: self.x, y: self.y - 1 }
    }

    pub fn south(&self) -> Coord {
        Self { x: self.x, y: self.y + 1 }
    }

    pub fn east(&self) -> Coord {
        Self { x: self.x + 1, y: self.y }
    }

    pub fn west(&self) -> Coord {
        Self { x: self.x - 1, y: self.y }
    }
}

#[derive(Copy, Clone, Debug)]
struct Blizzard {
    position: Coord,
    direction: Direction,
}

struct Valley {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    pub fn from_map(map: &str) -> Self {
        let blizzards =
            map.lines().enumerate().flat_map(|(y, line)|
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let dir = match c {
                        '^' => Some(Direction::North),
                        '>' => Some(Direction::East),
                        '<' => Some(Direction::West),
                        'v' => Some(Direction::South),
                        _ => None
                    };
                    dir.map(|direction| {
                        let position = Coord { x, y };
                        Blizzard { position, direction }
                    })
                })).collect();
        let width = map.lines().nth(0).unwrap().len();
        let height = map.lines().count();
        Self { blizzards, width, height }
    }

    fn tick(&mut self) -> HashSet<Coord> {
        self.blizzards.iter_mut()
            .update(|b| b.tick(self.width, self.height))
            .map(|b| b.position)
            .collect()
    }

    pub fn is_wall(&self, coord: &Coord) -> bool {
        coord.y >= self.height || coord.x >= self.width
            || coord.x == self.width - 1 || coord.x == 0
            || (coord.y == 0 && coord.x > 1)
            || (coord.y == self.height - 1 && coord.x < self.width - 2)
    }

    pub fn next_frontier(&mut self, prev: HashSet<Coord>) -> HashSet<Coord> {
        let blocked = self.tick();
        prev.iter().flat_map(|c| {
            let mut succs = Vec::new();
            let mut try_move = |sc| { if !self.is_wall(&sc) && !blocked.contains(&sc) { succs.push(sc); } };
            if c.y > 0 { try_move(c.north()); }
            try_move(c.east());
            if c.x > 0 { try_move(c.west()); }
            try_move(c.south());
            try_move(*c);
            succs
        }).collect()
    }
}


impl Blizzard {
    fn tick(&mut self, width: usize, height: usize) {
        match self.direction {
            Direction::North => {
                self.position.y -= 1;
                if self.position.y == 0 {
                    self.position.y = height - 2;
                }
            },
            Direction::South => {
                self.position.y += 1;
                if self.position.y == height - 1 {
                    self.position.y = 1;
                }
            },
            Direction::East => {
                self.position.x += 1;
                if self.position.x == width - 1 {
                    self.position.x = 1;
                }
            },
            Direction::West => {
                self.position.x -= 1;
                if self.position.x == 0 {
                    self.position.x = width - 2;
                }
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut input = String::new();
    cli.input.get_input()?.read_to_string(&mut input)?;

    let mut valley = Valley::from_map(input.as_str());

    let origin = Coord { x: 1, y: 0 };
    let target = Coord { y: valley.height - 1, x: valley.width - 2 };

    dbg!(valley.width, valley.height, origin, target);
    let mut frontier = HashSet::from([origin]);
    let mut steps = 0;

    loop {
        frontier = valley.next_frontier(frontier);
        steps += 1;
        if frontier.is_empty() { bail!("Frontier became empty!") }
        if frontier.contains(&target) { break; }
        dbg!(steps, frontier.len());
    }

    println!("Escape in {} steps", steps);

    frontier = HashSet::from([target]);
    loop {
        frontier = valley.next_frontier(frontier);
        steps +=1 ;
        if frontier.is_empty() {bail!("Frontier became empty!") }
        if frontier.contains(&origin) { break; }
        dbg!(steps, frontier.len());
    }

    println!("Return in {} steps", steps);

    frontier = HashSet::from([origin]);
    loop {
        frontier = valley.next_frontier(frontier);
        steps += 1;
        if frontier.is_empty() { bail!("Frontier became empty!") }
        if frontier.contains(&target) { break; }
        dbg!(steps, frontier.len());
    }

    println!("Escape again in {} steps", steps);


    Ok(())
}
