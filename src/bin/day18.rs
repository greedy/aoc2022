use std::collections::HashSet;

use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<18>
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord3 {
    fn neighbors(&self) -> [Coord3; 6] {
        [Coord3{x: self.x - 1, ..*self},
         Coord3{x: self.x + 1, ..*self},
         Coord3{y: self.y - 1, ..*self},
         Coord3{y: self.y + 1, ..*self},
         Coord3{z: self.z - 1, ..*self},
         Coord3{z: self.z + 1, ..*self}]
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut cubes: Vec<Coord3> = vec![];

    for line in cli.input.get_input()?.lines() {
        let line = line?;
        if let Some((Ok(x),Ok(y),Ok(z))) = line.split(",").map(|s| s.parse()).collect_tuple() {
            cubes.push(Coord3 { x, y, z })
        } else {
            bail!("Line {} is malformed", line)
        }
    }

    let mut cubes_set = HashSet::new();
    cubes_set.extend(cubes.iter().copied());

    let mut exposed_faces = 0;

    for cube in cubes_set.iter() {
        for neighbor in cube.neighbors() {
            if !cubes_set.contains(&neighbor) {
                exposed_faces += 1;
            }
        }
    }

    let min_x = cubes.iter().map(|c| c.x).min().unwrap() - 2;
    let max_x = cubes.iter().map(|c| c.x).max().unwrap() + 2;
    let min_y = cubes.iter().map(|c| c.y).min().unwrap() - 2;
    let max_y = cubes.iter().map(|c| c.y).max().unwrap() + 2;
    let min_z = cubes.iter().map(|c| c.z).min().unwrap() - 2;
    let max_z = cubes.iter().map(|c| c.z).max().unwrap() + 2;

    let in_bbox = |c: &Coord3| -> bool {
        c.x >= min_x && c.x <= max_x && c.y >= min_y && c.y <= max_y && c.z >= min_z && c.z <= max_z
    };

    println!("surface area is {}", exposed_faces);

    let mut external_coords = HashSet::from([Coord3{x:max_x-1,y:max_y-1,z:max_z-1}]);
    let mut open = vec![external_coords.iter().next().unwrap().clone()];

    while let Some(ex) = open.pop() {
        ex.neighbors().iter().filter(|c| in_bbox(&c)).filter(|c| !cubes_set.contains(&c)).for_each(|c| {
            if external_coords.insert(c.clone()) {
                open.push(c.clone());
            }
        });
    }

    let mut really_exposed_faces = 0;

    for cube in cubes_set.iter() {
        for neighbor in cube.neighbors() {
            if external_coords.contains(&neighbor) {
                really_exposed_faces += 1;
            }
        }
    }

    println!("external surface area is {}", really_exposed_faces);

    Ok(())
}
