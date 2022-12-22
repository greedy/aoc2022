use aoc2022::bitset::BitSet;
use itertools::Itertools;
use crate::caverns::*;
use std::{collections::{HashSet, HashMap}, cmp::Reverse};
use petgraph::{prelude::*, visit::{IntoNodeReferences, NodeRef}};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Agent {
    Me,
    TheElephant
}

impl Agent {
    pub fn the_other_one(self) -> Self {
        match self {
            Agent::Me => Agent::TheElephant,
            Agent::TheElephant => Agent::Me
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Action {
    who: Agent,
    action: crate::statespace::Action,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    my_state: crate::statespace::State,
    elephant_state: crate::statespace::State,
}

impl State {

    pub fn active_agent(&self) -> Agent {
        if self.my_state.time <= self.elephant_state.time {
            Agent::Me
        } else {
            Agent::TheElephant
        }
    }

    pub fn idle_agent(&self) -> Agent {
        self.active_agent().the_other_one()
    }

    pub fn agent_state(&self, agent: Agent) -> &crate::statespace::State {
        match agent {
            Agent::Me => &self.my_state,
            Agent::TheElephant => &self.elephant_state
        }
    }

    pub fn agent_state_mut(&mut self, agent: Agent) -> &mut crate::statespace::State {
        match agent {
            Agent::Me => &mut self.my_state,
            Agent::TheElephant => &mut self.elephant_state
        }
    }

    pub fn active_state(&self) -> &crate::statespace::State {
        self.agent_state(self.active_agent())
    }

    pub fn active_state_mut(&mut self) -> &mut crate::statespace::State {
        self.agent_state_mut(self.active_agent())
    }

    pub fn idle_state(&self) -> &crate::statespace::State {
        self.agent_state(self.idle_agent())
    }

    pub fn idle_state_mut(&mut self) -> &mut crate::statespace::State {
        self.agent_state_mut(self.idle_agent())
    }

    pub fn opening_valves(&self) -> u64 {
        self.my_state.open_valves | self.elephant_state.open_valves
    }

    pub fn min_time(&self) -> u32 {
        std::cmp::min(self.my_state.time, self.elephant_state.time)
    }

    pub fn max_time(&self) -> u32 {
        std::cmp::max(self.my_state.time, self.elephant_state.time)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Step {
    from: State,
    to: State,
    action: Action,
}

const ERUPTION_TIME : u32 = 30;

pub struct StateSpace<'a> {
    inner: crate::statespace::StateSpace<'a>,
    initial_state: State,
    max_score: u32,
}

impl<'a> StateSpace<'a> {
    pub fn new(caverns: &'a Caverns) -> Self {
        let inner = crate::statespace::StateSpace::new(caverns);
        let initial_state = {
            let mut my_state = inner.initial_state().clone();
            my_state.time = 4;
            let mut elephant_state = inner.initial_state().clone();
            elephant_state.time = 4;
            State { my_state, elephant_state }
        };
        let max_score = caverns.rooms().map(|room| room.valve_rate() * (ERUPTION_TIME - 4)).sum();
        Self { inner, initial_state, max_score }
    }

    pub fn initial_state(&self) -> &State {
        &self.initial_state
    }

    pub fn max_score(&self) -> u32 {
        self.max_score
    }

    pub fn node_for_id(&self, room_id: usize) -> NodeIndex<u32> {
        self.inner.node_for_id(room_id)
    }

    pub fn travel_time(&self, from_room_id: usize, to_room_id: usize) -> u32 {
        self.inner.travel_time(from_room_id, to_room_id)
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

    impl NodeRef for State {
        type NodeId = State;
        type Weight = State;

        fn id(&self) -> Self::NodeId {
            *self
        }

        fn weight(&self) -> &Self::Weight {
            self
        }
    }

    impl<'a> IntoNodeReferences for &'a StateSpace<'a> {
        type NodeRef = State;
        type NodeReferences = AllNodes<'a>;

        fn node_references(self) -> Self::NodeReferences {
            AllNodes::new(self)
        }
    }

    impl<'a> IntoNodeIdentifiers for &'a StateSpace<'a> {
        type NodeIdentifiers = AllNodeIds<'a>;

        fn node_identifiers(self) -> Self::NodeIdentifiers {
            AllNodeIds(self.node_references())
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

    pub struct AllNodeIds<'a>(AllNodes<'a>);

    impl<'a> Iterator for AllNodeIds<'a> {
        type Item = State;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
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

    pub struct AllNodes<'a> {
        state_space: &'a StateSpace<'a>,
        seen: HashSet<State>,
        open: Vec<State>,
    }

    impl<'a> AllNodes<'a> {
        fn new(state_space: &'a StateSpace) -> Self {
            let seen = HashSet::from([state_space.initial_state]);
            let open = vec![state_space.initial_state];
            Self { state_space, seen, open }
        }
    }

    impl<'a> Iterator for AllNodes<'a> {
        type Item = State;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(node) = self.open.pop() {
                for neighbor in self.state_space.neighbors(node) {
                    if self.seen.insert(neighbor) {
                        self.open.push(neighbor)
                    }
                }
                Some(node)
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
        my_action_iter: <&'a crate::statespace::StateSpace<'a> as IntoEdges>::Edges,
        elephant_action_iter: <&'a crate::statespace::StateSpace<'a> as IntoEdges>::Edges,
        who: Agent,
    }

    impl<'a> StepsFrom<'a> {
        fn new(state_space: &'a StateSpace<'a>, from: State) -> Self {
            let my_action_iter = state_space.inner.edges(from.my_state);
            let elephant_action_iter = state_space.inner.edges(from.elephant_state);
            let who = Agent::Me;
            Self { state_space, from, my_action_iter, elephant_action_iter, who }
        }
    }

    impl<'a> Iterator for StepsFrom<'a> {
        type Item = Step;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                match self.who {
                    Agent::Me => {
                        while let Some(inner_step) = self.my_action_iter.next() {
                            // no time travel
                            if inner_step.to.time < self.from.max_time() {
                                continue;
                            }
                            let mut to = self.from.clone();
                            to.my_state = inner_step.to;
                            to.my_state.open_valves = to.opening_valves();
                            to.elephant_state.open_valves = to.opening_valves();
                            let step = Step { from: self.from, to: to, action: Action { who: self.who, action: inner_step.action } };
                            return Some(step);
                        }
                        self.who = Agent::TheElephant;
                    },
                    Agent::TheElephant => {
                        while let Some(inner_step) = self.elephant_action_iter.next() {
                            // no time travel
                            if inner_step.to.time < self.from.max_time() {
                                continue;
                            }
                            let mut to = self.from.clone();
                            to.elephant_state = inner_step.to;
                            to.my_state.open_valves = to.opening_valves();
                            to.elephant_state.open_valves = to.opening_valves();
                            let step = Step { from: self.from, to: to, action: Action { who: self.who, action: inner_step.action } };
                            return Some(step);
                        }
                        break None;
                    }
                }
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

/* 
 *
 * pressure build-up is tricky
 *
 * 0, 0 -> 3, 0 => no pressure buildup until elephant moves
 * 3, 0 -> 3, 8 => pressure builds for three minutes using valves of from.elephant.valves
 * 3, 8 -> 6, 8 => pressure builds for three minutes using valves of from.me.valves
 * 6, 8 -> 9, 8 => pressure builds for two minutes using valves of from.me.valves
 */

impl Step {

    pub fn elapsed_time(&self) -> u32 {
        self.to.max_time() - self.from.max_time()
    }

    pub fn pressure_buildup<'a>(&self, state_space: &StateSpace<'a>) -> u32 {
        let pressure_rate : u32 = state_space.inner.valve_room_ids().iter().map(|id| {
            if self.from.opening_valves().contains(*id as u32) {
                0
            } else {
                state_space.inner.caverns().room(*id).valve_rate()
            }
        }).sum();
        let cost = pressure_rate * self.elapsed_time();
        //dbg!(self, cost);
        cost
    }
}

impl State {
    pub fn path_heuristic(&self, caverns: &Caverns) -> u32 {
        // cannot over-estimate remaining cost
        let mut closed_valves = caverns.rooms().enumerate().filter(|(id, room)| !self.opening_valves().contains(*id as u32)).map(|(_id, room)| room.valve_rate()).collect_vec();
        closed_valves.sort_by_key(|x| std::cmp::Reverse(*x));
        let cost = (0..(26 - self.min_time())).zip(closed_valves.chunks(2)).map(|(ticks, chunk)| chunk.iter().map(|rate| (*rate)*ticks).sum::<u32>()).sum();
        //dbg!(self, cost);
        cost
    }
}
