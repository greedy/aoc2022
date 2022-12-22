use aoc2022::bitset::BitSet;
use itertools::Itertools;
use crate::caverns::*;
use std::collections::{HashSet, HashMap};
use petgraph::{prelude::*, visit::{IntoNodeReferences, NodeRef}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Move,
    OpenValve,
    Wait { ticks: u32 },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time: u32,
    pub room_id: usize,
    pub open_valves: u64
}

impl State {

    pub fn time(&self) -> u32 { self.time }
    pub fn room_id(&self) -> usize { self.room_id }

    pub fn can_open_valve(&self) -> bool {
        !self.open_valves.contains(self.room_id as u32)
    }

    pub fn move_to(&self, space: &StateSpace, next_room: usize) -> State {
        let mut open_valves = self.open_valves.clone();
        open_valves.insert(next_room as u32);
        State {
            time: self.time + 1 + space.path_lengths[&(space.room_node_ids[self.room_id], space.room_node_ids[next_room])],
            room_id: next_room,
            open_valves,
        }
    }

    pub fn open_valve(&self) -> State {
        assert!(self.can_open_valve());
        let mut open_valves = self.open_valves.clone();
        open_valves.insert(self.room_id as u32);
        State {
            time: self.time + 1,
            room_id: self.room_id,
            open_valves
        }
    }

    pub fn move_to_step(&self, space: &StateSpace, next_room: usize) -> Step {
        let from = self.clone();
        let to = self.move_to(space, next_room);
        let action = Action::Move;
        Step { from, to, action }
    }

    pub fn open_valve_step(&self) -> Step {
        let from = self.clone();
        let to = self.open_valve();
        let action = Action::OpenValve;
        Step { from, to, action }
    }

    pub fn wait(&self, ticks: u32) -> State {
        State {
            time: self.time + ticks,
            room_id: self.room_id,
            open_valves: self.open_valves
        }
    }

    pub fn wait_step(&self, ticks: u32) -> Step {
        let from = self.clone();
        let to = self.wait(ticks);
        let action = Action::Wait { ticks };
        Step { from, to, action }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Step {
    pub from: State,
    pub to: State,
    pub action: Action,
}

const ERUPTION_TIME : u32 = 30;

pub struct StateSpace<'a> {
    caverns: &'a Caverns,
    cavern_graph: CavernGraph<'a>,
    path_lengths: HashMap<(NodeIndex<u32>, NodeIndex<u32>), u32>,
    room_node_ids: Vec<NodeIndex<u32>>,
    valve_room_ids: Vec<usize>,
    initial_state: State,
    max_score: u32,
}

impl<'a> StateSpace<'a> {
    pub fn new(caverns: &'a Caverns) -> Self {
        let max_score = caverns.rooms().map(|room| room.valve_rate() * ERUPTION_TIME).sum();
        let initial_state = {
            let room_id = caverns.room_id("AA").expect("Room named AA");
            let time = 0;
            let open_valves = 0;
            State { time, room_id, open_valves }
        };
        let cavern_graph = caverns.build_graph();
        let valve_room_ids = caverns.rooms().enumerate().filter_map(|(id, r)| if r.valve_rate() > 0 { Some(id) } else { None }).collect();
        let path_lengths = cavern_graph.shortest_paths();
        let mut room_node_ids = vec![NodeIndex::end(); cavern_graph.graph().node_count()];
        cavern_graph.graph().node_references().for_each(|noderef| {
            room_node_ids[noderef.weight().0] = noderef.id();
        });
        Self { max_score, caverns, cavern_graph, valve_room_ids, initial_state, path_lengths, room_node_ids }
    }

    pub fn caverns(&self) -> &'a Caverns {
        self.caverns
    }

    pub fn valve_room_ids(&self) -> &[usize] {
        self.valve_room_ids.as_slice()
    }

    pub fn initial_state(&self) -> &State {
        &self.initial_state
    }

    pub fn max_score(&self) -> u32 {
        self.max_score
    }
    
    pub fn node_for_id(&self, room_id: usize) -> NodeIndex<u32> {
        self.room_node_ids[room_id]
    }

    pub fn travel_time(&self, from_room_id: usize, to_room_id: usize) -> u32 {
        self.path_lengths[&(self.node_for_id(from_room_id), self.node_for_id(to_room_id))]
    }
}

mod graph_traits {
    use std::ops::RangeInclusive;

    use petgraph::adj::Neighbors;
    use petgraph::prelude::*;
    use petgraph::visit::*;
    use super::*;

    impl<'a> GraphBase for StateSpace<'a> {
        type EdgeId = Step;
        type NodeId = State;
    }

    impl<'a> GraphProp for StateSpace<'a> {
        type EdgeType = Directed;
    }

    impl<'a> Data for StateSpace<'a> {
        type NodeWeight = Self::NodeId;
        type EdgeWeight = Self::EdgeId;
    }

    impl<'a> IntoNeighbors for &'a StateSpace<'a> {
        type Neighbors = NeighborsFrom<'a>;

        fn neighbors(self, a:Self::NodeId) -> Self::Neighbors {
            NeighborsFrom::new(self, a)
        }
    }

    impl<'a> EdgeRef for Step {
        type NodeId = State;
        type EdgeId = Step;
        type Weight = Step;

        fn source(&self) -> State {
            self.from
        }

        fn target(&self) -> State {
            self.to
        }

        fn weight(&self) -> &Step {
            self
        }

        fn id(&self) -> Step {
            *self
        }
    }

    impl<'a> IntoEdgeReferences for &'a StateSpace<'a> {
        type EdgeRef = Step;
        type EdgeReferences = AllEdges<'a>;

        fn edge_references(self) -> Self::EdgeReferences {
            AllEdges::new(self)
        }
    }

    impl<'a> IntoEdges for &'a StateSpace<'a> {
        type Edges = StepsFrom<'a>;

        fn edges(self, a: Self::NodeId) -> Self::Edges {
            StepsFrom::new(self, a)
        }
    }

    impl<'a> Visitable for StateSpace<'a> {
        type Map = HashSet<State>;

        fn visit_map(self: &Self) -> Self::Map {
            HashSet::new()
        }

        fn reset_map(self: &Self,map: &mut Self::Map) {
            map.clear()
        }
    }

    pub struct AllEdges<'a> {
        state_space: &'a StateSpace<'a>,
        seen: HashSet<State>,
        open: Vec<State>,
        step_iter: StepsFrom<'a>
    }

    impl<'a> AllEdges<'a> {
        fn new(state_space: &'a StateSpace) -> Self {
            let seen = HashSet::from([state_space.initial_state]);
            let open = vec![];
            let step_iter = StepsFrom::new(state_space, state_space.initial_state);
            Self { state_space, seen, open, step_iter }
        }
    }

    impl<'a> Iterator for AllEdges<'a> {
        type Item = Step;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(step) = self.step_iter.next() {
                if self.seen.insert(step.to) {
                    self.open.push(step.to)
                }
                return Some(step)
            } else if let Some(open_state) = self.open.pop() {
                self.step_iter = StepsFrom::new(self.state_space, open_state);
                self.next()
            } else {
                None
            }
        }
    }

    enum NeighborStage {
        EmitOpen,
        EmitMoves,
        EmitWaits,
    }

    pub struct StepsFrom<'a> {
        state_space: &'a StateSpace<'a>,
        from: State,
        stage: NeighborStage,
        connected_rooms_iter: Box<dyn Iterator<Item = usize> + 'a>,
        wait_ticks: RangeInclusive<u32>,
    }

    impl<'a> StepsFrom<'a> {
        fn new(state_space: &'a StateSpace<'a>, from: State) -> Self {
            let stage = NeighborStage::EmitMoves;
            let open_valves = from.open_valves;
            let connected_rooms_iter = Box::new(state_space.valve_room_ids.iter().copied().filter(move |vid| !open_valves.contains(*vid as u32)));
            //let wait_ticks = u32::max(1, u32::saturating_sub(26, from.time()))..=(30 - from.time());
            let wait_ticks = (30 - from.time())..=(30 - from.time());
            Self { state_space, from, stage, connected_rooms_iter, wait_ticks }
        }
    }

    impl<'a> Iterator for StepsFrom<'a> {
        type Item = Step;

        fn next(&mut self) -> Option<Self::Item> {
            if self.from.time >= ERUPTION_TIME { return None }
            match self.stage {
                NeighborStage::EmitOpen => unreachable!(),
                NeighborStage::EmitMoves => {
                    if let Some(step) = self.connected_rooms_iter.next().map(|next_room_id| self.from.move_to_step(self.state_space, next_room_id)) {
                        Some(step)
                    } else {
                        self.stage = NeighborStage::EmitWaits;
                        self.next()
                    }
                },
                NeighborStage::EmitWaits => {
                    self.wait_ticks.next().map(|ticks| self.from.wait_step(ticks))
                },
            }
        }
    }

    pub struct NeighborsFrom<'a>(StepsFrom<'a>);
    
    impl<'a> NeighborsFrom<'a> {
        fn new(state_space: &'a StateSpace<'a>, from: State) -> Self {
            Self(StepsFrom::new(state_space, from))
        }
    }

    impl<'a> Iterator for NeighborsFrom<'a> {
        type Item = State;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|step| step.to)
        }
    }

}

pub use graph_traits::*;

impl State {
    pub fn pressure_buildup(&self, caverns: &Caverns) -> u32 {
        let cost = caverns.rooms().enumerate().map(|(id, room)| {
            if self.open_valves.contains(id as u32) {
                0
            } else {
                room.valve_rate()
            }
        }).sum();
        cost
    }

    pub fn path_heuristic(&self, caverns: &Caverns) -> u32 {
        // cannot over-estimate remaining cost
        let mut closed_valves = caverns.rooms().enumerate().filter(|(id, room)| !self.open_valves.contains(*id as u32)).map(|(_id, room)| room.valve_rate()).collect_vec();
        closed_valves.sort();
        let cost = (1..(30 - self.time())).zip(closed_valves.iter().rev()).map(|(ticks, rate)| rate*ticks).sum();
        //dbg!(self, cost);
        cost
    }
}
