use aoc2022::prelude::*;
use std::collections::{HashMap,HashSet};

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<23>
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: isize,
    y: isize
}

#[derive(Copy, Clone, Debug)]
struct Rectangle {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Rectangle {
    fn extend_to_include(&mut self, coord: &Coord) {
        self.min_x = self.min_x.min(coord.x);
        self.max_x = self.max_x.max(coord.x);
        self.min_y = self.min_y.min(coord.y);
        self.max_y = self.max_y.max(coord.y);
    }

    fn degenerate() -> Self {
        let min_x = isize::MAX;
        let max_x = isize::MIN;
        let min_y = isize::MAX;
        let max_y = isize::MIN;
        Self { min_x, max_x, min_y, max_y }
    }

    fn area(&self) -> usize {
        if self.max_x < self.min_x || self.max_y < self.min_y { 0 }
        else {
            let width: usize = (self.max_x - self.min_x + 1).try_into().unwrap();
            let height: usize = (self.max_y - self.min_y + 1).try_into().unwrap();
            width * height
        }
    }
}

impl Coord {
    fn north(&self) -> Self {
        Coord { y: self.y - 1, x: self.x }
    }

    fn south(&self) -> Self {
        Coord { y: self.y + 1, x: self.x }
    }

    fn east(&self) -> Self {
        Coord { x: self.x + 1, y: self.y }
    }

    fn west(&self) -> Self {
        Coord { x: self.x - 1, y: self.y }
    }

    fn northeast(&self) -> Self {
        self.north().east()
    }

    fn northwest(&self) -> Self {
        self.north().west()
    }

    fn southwest(&self) -> Self {
        self.south().west()
    }

    fn southeast(&self) -> Self {
        self.south().east()
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

impl std::ops::Add<Direction> for &Coord {
    type Output = Coord;

    fn add(self, dir: Direction) -> Coord {
        match dir {
            Direction::North => self.north(),
            Direction::South => self.south(),
            Direction::West => self.west(),
            Direction::East => self.east()
        }
    }
}

struct Grove {
    elf_positions: HashSet<Coord>,
    proposed_moves: HashMap<Coord, Vec<Coord>>,
    direction_order: [Direction; 4],
}

impl Grove {
    pub fn from_map(map: &str) -> Self {
        let elf_positions = (0isize..).zip(map.lines()).flat_map(|(y, line)|
            (0isize..).zip(line.chars()).filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Coord { x, y })
                } else {
                    None
                }
            })).collect();
        let direction_order = [ Direction::North, Direction::South, Direction::West, Direction::East ];
        let proposed_moves = HashMap::new();
        Self { elf_positions, direction_order, proposed_moves }
    }

    fn need_to_move(&self, elf: &Coord) -> bool {
        let probes = [
            elf.northwest(), elf.north(), elf.northeast(),
            elf.west(),                   elf.east(),
            elf.southwest(), elf.south(), elf.southeast()
        ];
        probes.iter().any(|p| self.elf_positions.contains(p))
    }

    fn evaluate_move(&self, elf: &Coord, direction: Direction) -> bool {
        let probes =
            match direction {
                Direction::North => [elf.north(), elf.northeast(), elf.northwest()],
                Direction::South => [elf.south(), elf.southeast(), elf.southwest()],
                Direction::West => [elf.west(), elf.northwest(), elf.southwest()],
                Direction::East => [elf.east(), elf.northeast(), elf.southeast()],
            };
        probes.iter().all(|p| !self.elf_positions.contains(p))
    }

    fn propose(&mut self) {
        self.proposed_moves.clear();
        for elf in self.elf_positions.iter() {
            if !self.need_to_move(elf) {
                self.proposed_moves.insert(*elf, vec![*elf]);
            } else {
                let mut moved = false;
                for dir in self.direction_order.iter().copied() {
                    if self.evaluate_move(elf, dir) {
                        let proposal = elf + dir;
                        let elfs = match self.proposed_moves.get_mut(&proposal) {
                            Some(elfs) => elfs,
                            None => {
                                let elfs = Vec::new();
                                self.proposed_moves.insert(proposal, elfs);
                                self.proposed_moves.get_mut(&proposal).unwrap()
                            }
                        };
                        elfs.push(*elf);
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    self.proposed_moves.insert(*elf, vec![*elf]);
                }
            }
        }
    }

    fn apply_proposal(&mut self) {
        let mut next = HashSet::new();
        for (proposal, elfs) in self.proposed_moves.iter() {
            if elfs.len() == 1 {
                next.insert(*proposal);
            } else {
                next.extend(elfs);
            }
        }
        self.elf_positions = next;
    }

    fn round(&mut self) {
        self.propose();
        self.apply_proposal();
        self.direction_order.as_mut_slice().rotate_left(1);
    }

    fn occupied_rectangle(&self) -> Rectangle {
        self.elf_positions.iter().fold(Rectangle::degenerate(), |mut r, c| { r.extend_to_include(c); r})
    }

    fn elf_count(&self) -> usize {
        self.elf_positions.len()
    }

    fn dump(&self) -> std::io::Result<()> {
        let rect = self.occupied_rectangle();
        let mut out = std::io::stdout();
        for y in rect.min_y-2..=rect.max_y+2 {
            for x in rect.min_x-2..=rect.max_x+2 {
                if self.elf_positions.contains(&Coord { x, y }) {
                    out.write(&[b'#'])?;
                } else {
                    out.write(&[b'.'])?;
                }
            }
            out.write(&[b'\n'])?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut input = String::new();
    cli.input.get_input()?.read_to_string(&mut input)?;

    let mut grove = Grove::from_map(input.as_str());
    println!("Intitial state");
    grove.dump()?;

    for round_num in 1..=10 {
        grove.round();
        println!("After round {}", round_num);
        grove.dump()?;
    }

    let empty_tiles = grove.occupied_rectangle().area() - grove.elf_count();

    println!("{} empty tiles", empty_tiles);

    Ok(())
}
