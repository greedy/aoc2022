use std::{ops::Index, fmt::Display};

use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<20>
}

#[derive(Debug)]
struct Num {
    data: i64,
    prev: isize,
    next: isize,
}

#[derive(Debug)]
struct File(Vec<Num>);

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::iter::successors(Some(0), |ix:&isize| Some(self.at(*ix).next)).map(|ix| self.at(ix).data).take(self.0.len()).join(", "))
    }
}

impl File {
    fn advance(&mut self, a: isize) {
        let b = self.at(a).next;
        self.at_mut(self.at(a).prev).next = b;
        self.at_mut(self.at(b).next).prev = a;
        self.at_mut(a).next = self.at(b).next;
        self.at_mut(b).prev = self.at(a).prev;
        self.at_mut(a).prev = b;
        self.at_mut(b).next = a;
    }

    fn retreat(&mut self, a: isize) {
        self.advance(self.at(a).prev)
    }

    fn resolve_signed_index(&self, index: isize) -> usize {
        if index < 0 { self.0.len() - (index.unsigned_abs() % self.0.len()) }
        else { index.unsigned_abs() % self.0.len() }
    }

    fn at(&self, index: isize) -> &Num {
        let actual = self.resolve_signed_index(index);
        &self.0[actual]
    }

    fn at_mut(&mut self, index: isize) -> &mut Num {
        let actual = self.resolve_signed_index(index);
        &mut self.0[actual]
    }

    fn mix(&mut self) {
        //println!("Before mixing: {}", &self);
        for orig_ix in 0..self.0.len() {
            let orig_ix: isize = orig_ix.try_into().expect("indices fit in isize");
            let data = self.at(orig_ix).data;
            let data_abs: usize = data.unsigned_abs().try_into().unwrap();
            if data < 0 {
                for _ in 0..(data_abs % (self.0.len() - 1)) {
                    self.retreat(orig_ix);
                }
            } else {
                for _ in 0..(data_abs % (self.0.len() - 1)) {
                    self.advance(orig_ix);
                }
            }
            //println!("Round {}: {}", orig_ix+1, &self);
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let numbers: Vec<_> = cli.input.get_input()?.lines().into_eyre().map_and_then(|s| s.parse::<i64>().map_err(Into::into)).try_collect()?;

    {
    let mut file = File(numbers.iter().copied().zip(0isize..).map(|(data, ix)| Num { data, next: ix + 1, prev: ix - 1 }).collect_vec());

    println!("input has {} numbers", file.0.len());

    file.mix();

    let zero_ix:isize = file.0.iter().position(|n| n.data == 0).expect("file to contain a zero").try_into().expect("indicies to fit in isize");


    let mut ii = std::iter::successors(Some(zero_ix), |ix:&isize| Some(file.at(*ix).next)).map(|ix| file.at(ix).data);;

    //println!("Mixed list is {}", ii.clone().take(file.0.len()*2).join(", "));

    let first = ii.nth(1000).unwrap();
    let second = ii.nth(999).unwrap();
    let third = ii.nth(999).unwrap();

    dbg!(first, second, third);

    println!("Result is {}", first+second+third);
    }

    {

    let key = 811589153;

    let mut file = File(numbers.iter().copied().map(|x| x  * key).zip(0isize..).map(|(data, ix)| Num { data, next: ix + 1, prev: ix - 1 }).collect_vec());

    //println!("{}", std::iter::successors(Some(0), |ix:&isize| Some(file.at(*ix).next)).map(|ix| file.at(ix).data).take(file.0.len()).join(", "));
    for i in 0..10 {
        println!("mixing round {}", i+1);
        file.mix();
        //println!("{}", std::iter::successors(Some(0), |ix:&isize| Some(file.at(*ix).next)).map(|ix| file.at(ix).data).take(file.0.len()).join(", "));
    }
    let zero_ix:isize = file.0.iter().position(|n| n.data == 0).expect("file to contain a zero").try_into().expect("indicies to fit in isize");


    let mut ii = std::iter::successors(Some(zero_ix), |ix:&isize| Some(file.at(*ix).next)).map(|ix| file.at(ix).data);

    //println!("Mixed list is {}", ii.clone().take(file.0.len()*2).join(", "));

    let first = ii.nth(1000).unwrap();
    let second = ii.nth(999).unwrap();
    let third = ii.nth(999).unwrap();

    dbg!(-1623178306 / 6);
    dbg!(-1623178306 % 6);

    dbg!(first, second, third);

    println!("Result is {}", first+second+third);
    }

    Ok(())
}

// a, b, c, d, e, f
// b, a, c, d, e, f
// b, c, a, d, e, f
// b, c, d, a, e, f
// b, c, d, e, a, f
// b, c, d, e, f, a
