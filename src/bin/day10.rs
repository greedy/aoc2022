use std::str::FromStr;
use color_eyre::eyre::{bail, Context};
use itertools::Itertools;

use aoc2022::prelude::*;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<10>
}

enum Instruction {
    Noop,
    Addx(i32)
}

impl Instruction {
    pub fn cycles(&self) -> u32 {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }

    fn execute_cycle(&self, communicator: &mut Communicator, cycle_number: u32) {
        assert!(cycle_number < self.cycles());
        match self {
            Self::Noop => (),
            Self::Addx(v) => {
                if cycle_number == 1 {
                    communicator.x_register += v
                }
            }
        }
        communicator.current_cycle += 1;
    }
}

impl FromStr for Instruction {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().peekable();
        let mnemonic = parts.next().ok_or_else(|| eyre!("Empty instruction!"))?;
        match mnemonic {
            "noop" => {
                if parts.peek().is_some() {
                    bail!("Unexpected arguments for 'noop': {}", parts.join(" "))
                }
                Ok(Instruction::Noop)
            },
            "addx" => {
                let v = parts.next().ok_or_else(|| eyre!("Missing operand for 'addx'"))?;
                if parts.peek().is_some() {
                    bail!("Extra operand for 'addx': {}", parts.join(" "));
                }
                let v = v.parse().wrap_err("Operand of 'addx' is not a number")?;
                Ok(Instruction::Addx(v))
            }
            _ => {
                bail!("Unknown instruction mnemonic: {}", mnemonic)
            }
        }
    }
}

struct Communicator {
    current_cycle: i32,
    x_register: i32
}

impl Communicator {
    pub fn new() -> Self {
        Self { current_cycle: 1, x_register: 1 }
    }

    pub fn execute<'a, 'i, II : Iterator<Item = &'i Instruction>>(&'a mut self, instructions: &'i mut II) -> Execution<'a, 'i, II> {
        Execution::new(self, instructions)
    }

    pub fn signal_strength(&self) -> i32 {
        self.current_cycle * self.x_register
    }

    pub fn reset(&mut self) {
        self.current_cycle = 1;
        self.x_register = 1;
    }
}

struct Execution<'a, 'i, II> {
    communicator: &'a mut Communicator,
    instructions: &'i mut II,
    current_instruction: Option<&'i Instruction>,
    instruction_cycle: u32,
}

impl<'a, 'i, II: Iterator<Item=&'i Instruction>> Execution<'a, 'i, II> {
    fn new(communicator: &'a mut Communicator, instructions: &'i mut II) -> Self {
        let instruction_cycle = 0;
        let current_instruction = instructions.next();
        Self { communicator, instructions, current_instruction, instruction_cycle }
    }
}

struct State {
    cycle: i32,
    x: i32
}

impl State {
    pub fn signal_strength(&self) -> i32 {
        self.cycle * self.x
    }

    pub fn of(c: &Communicator) -> Self {
        Self { cycle: c.current_cycle, x: c.x_register }
    }
}

impl<'a, 'i, II: Iterator<Item=&'i Instruction>> Iterator for Execution<'a, 'i, II> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_instruction {
            None => None,
            Some(insn) => {
                insn.execute_cycle(self.communicator, self.instruction_cycle);
                self.instruction_cycle += 1;
                if self.instruction_cycle >= insn.cycles() {
                    self.instruction_cycle = 0;
                    self.current_instruction = self.instructions.next();
                }
                Some(State::of(self.communicator))
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let program : Vec<_> = cli.input.get_input()?.lines().map(|l| l.map_err(|e| e.into()).and_then(|s| s.parse())).try_collect()?;

    let mut communicator = Communicator::new();

    let sum_of_strengths : i32 = communicator.execute(&mut program.iter())
        .filter(|s| s.cycle >= 20 && (s.cycle - 20) % 40 == 0)
        .inspect(|s| println!("Cycle {} Signal-stregth {}", s.cycle, s.signal_strength()))
        .map(|s| s.signal_strength())
        .sum();

    println!("Sum of signal strengths: {}", sum_of_strengths);

    communicator.reset();

    for scan_line in &(std::iter::once(State::of(&communicator)).chain(communicator.execute(&mut program.iter()))).chunks(40) {
        let line = scan_line.enumerate().map(|(pixel, s)| {
            if i32::abs((pixel as i32) - s.x) <= 1 {
                "#"
            } else {
                "."
            }
        }).join("");
        println!("{}", line);
    }
        

    Ok(())
}
