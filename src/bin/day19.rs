use std::str::FromStr;

use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<19>
}

#[derive(Copy, Clone, Debug)]
struct State {
    ore_rate: u32,
    clay_rate: u32,
    obsidian_rate: u32,
    geode_rate: u32,
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,
}

impl State {
    fn dominates(&self, other: &State) -> bool {
        self.ore_rate >= other.ore_rate
            && self.clay_rate >= other.clay_rate
            && self.obsidian_rate >= other.obsidian_rate
            && self.geode_rate >= other.geode_rate
            && self.ore_count >= other.ore_count
            && self.clay_count >= other.clay_count
            && self.obsidian_count >= other.obsidian_count
            && self.geode_count >= other.geode_count
    }

    const INITIAL: Self =
        Self {
            ore_rate: 1,
            clay_rate: 0,
            obsidian_rate: 0,
            geode_rate: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0
        };

    fn successors(&self, blueprint: &Blueprint) -> Vec<Self> {
        let mut succs = vec![*self];
        if self.ore_rate < blueprint.max_ore_consumption() && self.ore_count >= blueprint.ore_ore_cost {
            succs.push(Self { ore_count: self.ore_count - blueprint.ore_ore_cost, ore_rate: self.ore_rate + 1, ..*self });
        }
        if self.clay_rate < blueprint.max_clay_consumption() && self.ore_count >= blueprint.clay_ore_cost {
            succs.push(Self { ore_count: self.ore_count - blueprint.clay_ore_cost, clay_rate: self.clay_rate + 1, ..*self });
        }
        if self.obsidian_rate < blueprint.max_obsidian_consumption() && self.ore_count >= blueprint.obsidian_ore_cost && self.clay_count >= blueprint.obsidian_clay_cost {
            succs.push(Self {
                ore_count: self.ore_count - blueprint.obsidian_ore_cost,
                clay_count: self.clay_count - blueprint.obsidian_clay_cost,
                obsidian_rate: self.obsidian_rate + 1,
                ..*self
            });
        }
        if self.ore_count >= blueprint.geode_ore_cost && self.obsidian_count >= blueprint.geode_obsidian_cost {
            succs.push(Self {
                ore_count: self.ore_count - blueprint.geode_ore_cost,
                obsidian_count: self.obsidian_count - blueprint.geode_obsidian_cost,
                geode_rate: self.geode_rate + 1,
                ..*self
            });
        }
        for succ in succs.iter_mut() {
            succ.ore_count += self.ore_rate;
            succ.clay_count += self.clay_rate;
            succ.obsidian_count += self.obsidian_rate;
            succ.geode_count += self.geode_rate;
        }
        succs
    }
}

#[derive(Debug)]
struct StateCollection(Vec<State>);

impl StateCollection {
    fn add(&mut self, state: State) {
        if self.0.iter().any(|existing| existing.dominates(&state)) {
            return ()
        }
        self.0.retain(|existing| !state.dominates(existing));
        self.0.push(state);
    }

    fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> From<T> for StateCollection
where T: Into<Vec<State>>
{
    fn from(x: T) -> Self {
        Self(x.into())
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_ore_cost: u32,
    clay_ore_cost: u32,
    obsidian_ore_cost: u32,
    obsidian_clay_cost: u32,
    geode_ore_cost: u32,
    geode_obsidian_cost: u32,
}

impl FromStr for Blueprint {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, ore_ore_cost, clay_ore_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost) = s.split_ascii_whitespace().map(|s| s.trim_matches(|c:char| !c.is_ascii_digit())).filter(|s| !s.is_empty()).map(|s| s.parse::<u32>()).collect_tuple().ok_or_else(|| eyre!("Expected 7 numbers"))?;
        let (id, ore_ore_cost, clay_ore_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost) = (id?, ore_ore_cost?, clay_ore_cost?, obsidian_ore_cost?, obsidian_clay_cost?, geode_ore_cost?, geode_obsidian_cost?);
        Ok(Self { id, ore_ore_cost, clay_ore_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost, })
    }
}

impl Blueprint {
    fn is_overbuild(&self, st: &State) -> bool {
        st.ore_rate > self.max_ore_consumption()
            || st.clay_rate > self.max_clay_consumption()
            || st.obsidian_rate > self.max_obsidian_consumption()
    }

    fn max_ore_consumption(&self) -> u32 {
        u32::max(u32::max(self.geode_ore_cost, self.obsidian_ore_cost), u32::max(self.ore_ore_cost, self.clay_ore_cost))
    }

    fn max_clay_consumption(&self) -> u32 {
        self.obsidian_clay_cost
    }

    fn max_obsidian_consumption(&self) -> u32 {
        self.geode_obsidian_cost
    }
}
 
fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let blueprints: Vec<_> = cli.input.get_input()?.lines().into_eyre().map_and_then(|s| s.parse::<Blueprint>()).try_collect()?;

    let mut quality = 0;

    for blueprint in blueprints {
        dbg!(&blueprint);
        let mut states = vec![StateCollection::from([State::INITIAL])];
        for _minute in 1..=24 {
            let mut next_states = StateCollection::new();
            states.last().unwrap().0.iter().flat_map(|s| s.successors(&blueprint)).for_each(|s| next_states.add(s));
            dbg!(_minute, next_states.0.len());
            //dbg!(&next_states);
            states.push(next_states);
        }
        let max_geodes = states.last().unwrap().0.iter().map(|s| s.geode_count).max().unwrap();
        let this_quality = max_geodes * blueprint.id;
        println!("Blueprint {} produces {} geodes for {} quality", blueprint.id, max_geodes, this_quality);
        quality += this_quality;
    }

    println!("Total quality is {}", quality);

    Ok(())
}
