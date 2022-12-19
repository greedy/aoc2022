use aoc2022::prelude::*;
use itertools::{process_results, Itertools};
use petgraph::prelude::*;
use std::collections::HashMap;

mod caverns;
mod statespace;

use caverns::*;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<16>
}

/*
impl State {
    fn room<'a>(&self, caverns: &'a Caverns) -> &'a Room {
        &caverns.rooms[self.room_id]
    }
    fn successors(&self, caverns: &Caverns) -> impl Iterator<Item = (Action, State)> {
        let time = self.time + 1;
        let moves = caverns.neighbors(self.room(caverns)).map(move |room| {
            let room = &caverns.rooms[adj];
            let open_valves = self.open_valves.clone();
            (Action::Move, State { time, room, open_valves })
        });
        let open =
            std::iter::once(self.room).filter(|r| !self.open_valves.contains(r.name.as_str())).map(move |room| {
                let mut open_valves = self.open_valves.clone();
                open_valves.insert(self.room.name.as_str());
                (Action::OpenValve, State { time, room, open_valves })
            });
        open.chain(moves)
    }
}
*/

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut rooms = vec![];

    for line in cli.input.get_input()?.lines() {
        let line = line?;

        let room = Room::parse(&line).map_err(|e| e.to_owned())?.1;
        rooms.push(room);
    }

    let caverns = caverns::Caverns::new(rooms);
    let statespace = statespace::StateSpace::new(&caverns);

    // let costs = petgraph::algo::dijkstra(&statespace, *statespace.initial_state(), None, |e| e.source().pressure_buildup(&caverns));
    //

    let (cost, path) = petgraph::algo::astar(&statespace, *statespace.initial_state(), |st| st.time() == 30, |e| e.source().pressure_buildup(&caverns) * (e.target().time() - e.source().time()) , |st| st.path_heuristic(&caverns)).unwrap();

    let relief = statespace.max_score() - cost;

    for state in path.iter() {
        println!("{:?}", state);
    }

    println!("Pressure relief is {}", relief);
    Ok(())
}
