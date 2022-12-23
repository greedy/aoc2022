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
    cubes_set.extend(cubes);

    let mut exposed_faces = 0;

    for cube in cubes_set.iter() {
        if !cubes_set.contains(&Coord3{x: cube.x - 1, ..*cube}) {
            exposed_faces += 1;
        }
        if !cubes_set.contains(&Coord3{x: cube.x + 1, ..*cube}) {
            exposed_faces += 1;
        }
        if !cubes_set.contains(&Coord3{y: cube.y - 1, ..*cube}) {
            exposed_faces += 1;
        }
        if !cubes_set.contains(&Coord3{y: cube.y + 1, ..*cube}) {
            exposed_faces += 1;
        }
        if !cubes_set.contains(&Coord3{z: cube.z - 1, ..*cube}) {
            exposed_faces += 1;
        }
        if !cubes_set.contains(&Coord3{z: cube.z + 1, ..*cube}) {
            exposed_faces += 1;
        }
    }
    
    println!("surface area is {}", exposed_faces);

    Ok(())
}
