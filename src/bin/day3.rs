use clap::Parser;
use std::{io, ascii::AsciiExt};
use std::io::prelude::*;
use std::str::FromStr;
use color_eyre::eyre::{Report, Result, bail};
use itertools::{process_results, Itertools};

use aoc2022::inputs::InputCLI;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<3>
}

mod item {
    use color_eyre::eyre::{Report, Result, bail};
    use std::str::FromStr;

    #[derive(Clone, Copy, Debug)]
    pub struct Item {
        priority: u8
    }

    impl Item {
        pub fn priority(self) -> u8 { self.priority }
    }

    impl TryFrom<u8> for Item {
        type Error = Report;
        fn try_from(priority: u8) -> Result<Self> {
            if priority >= 63 {
                bail!("Priority {} is greater than maximum of 64", priority);
            }
            Ok(Item { priority })
        }
    }

    impl TryFrom<char> for Item {
        type Error = Report;
        fn try_from(c: char) -> Result<Self> {
            let priority : u8 =
                if c.is_ascii_uppercase() {
                    (c as u8) - ('A' as u8) + 27
                } else if c.is_ascii_lowercase() {
                    (c as u8) - ('a' as u8) + 1
                } else {
                    bail!("Item character must be ascii alphabetic");
                };
            Self::try_from(priority)
        }
    }

    impl FromStr for Item {
        type Err = Report;
        fn from_str(s: &str) -> Result<Self> {
            if s.len() != 1 {
                bail!("Item string must be one byte");
            }
            let c : char = s.chars().next().unwrap();
            Self::try_from(c)
        }
    }


    #[derive(Clone, Copy)]
    pub struct ItemSet(u64);

    impl Default for ItemSet {
        fn default() -> Self { ItemSet(0) }
    }

    impl ItemSet {
        pub fn add(&mut self, item: Item) {
            assert!(item.priority <= 63);
            self.0 |= 1 << item.priority
        }

        pub fn with(self, item: Item) {
            Self(self.0 | (1 << item.priority));
        }

        pub fn intersection(self, other: Self) -> Self {
            Self(self.0 & other.0)
        }

        pub fn intersect_with(&mut self, other: Self) {
            self.0 &= other.0
        }

        pub fn union(self, other: Self) -> Self {
            Self(self.0 | other.0)
        }

        pub fn union_with(&mut self, other: Self) {
            self.0 |= other.0
        }

        pub fn iter(self) -> SetIter {
            SetIter::new(self)
        }
    }

    #[derive(Debug)]
    pub struct SetIter(bit_iter::BitIter<u64>);

    impl SetIter {
        fn new(s: ItemSet) -> Self {
            SetIter(bit_iter::BitIter::from(s.0))
        }
    }

    impl Iterator for SetIter {
        type Item = Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next().map(|n| {
                let priority = n as u8;
                Item{priority}
            })
        }
    }

    impl IntoIterator for ItemSet {
        type Item = Item;
        type IntoIter = SetIter;
        fn into_iter(self) -> SetIter {
            self.iter()
        }
    }

    impl FromIterator<Item> for ItemSet {
        fn from_iter<I: IntoIterator<Item=Item>>(iter: I) -> Self {
            let mut c = Self::default();
            for i in iter {
                c.add(i);
            }
            c
        }
    }
}


#[derive(Clone, Copy)]
struct Rucksack {
    first_compartment: item::ItemSet,
    second_compartment: item::ItemSet
}

impl Rucksack {
    pub fn all_items(&self) -> item::ItemSet {
        self.first_compartment.union(self.second_compartment)
    }
}

impl<'a> FromStr for Rucksack {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() % 2 != 0 {
            bail!("Input string must be even length")
        }
        let (first, second) = s.split_at(s.len() / 2);
        let first_compartment = first.chars().map(item::Item::try_from).try_collect()?;
        let second_compartment = second.chars().map(item::Item::try_from).try_collect()?;
        Ok(Self { first_compartment, second_compartment })
    }
}


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let rucksacks : Vec<_> =
        cli.input.get_input()?.lines()
        .map(|r| r.map_err(Report::from))
        .map(|r| r.and_then(|s| Rucksack::from_str(&s.as_str())))
        .try_collect()?;


    let total_dup_priority : usize = 
        rucksacks.iter().flat_map(|r| r.first_compartment.intersection(r.second_compartment))
        .map(item::Item::priority)
        .map(usize::from)
        .sum();

    println!("Sum of the prorities of duplicate items is {total_dup_priority}");

    let badge_priority_total : usize = rucksacks.iter().chunks(3).into_iter()
        .map(|group| {
            group.map(Rucksack::all_items)
            .reduce(item::ItemSet::intersection)
            .unwrap()
            .iter()
            .exactly_one()
            .unwrap()
        })
        .map(item::Item::priority)
        .map(usize::from)
        .sum();

    println!("Sum of badge priorities is {badge_priority_total}");

    Ok(())
}
