use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<5>
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Crate {
    label: char
}

impl Crate {
    pub fn new(label: char) -> Self {
        Crate { label }
    }

    pub fn label(self) -> char {
        self.label
    }
}

#[derive(Clone, Debug)]
pub struct Ship {
    stacks: Vec<Vec<Crate>>
}

enum Mode {
    Part1,
    Part2
}

impl Ship {
    fn new(nstacks: usize) -> Self {
        let stacks = vec![vec![]; nstacks];
        Self { stacks }
    }
    fn execute(&mut self, step: &Step, mode: Mode) -> Result<()> {
        let (before_from, from_and_after) = self.stacks.split_at_mut(step.from_stack - 1);
        let (from_stack, after_from) =
            from_and_after.split_first_mut()
            .ok_or_else(|| eyre!("From stack {} does not exist", step.from_stack))?;
        let to_stack =
            if step.to_stack < step.from_stack {
                before_from.get_mut(step.to_stack - 1)
                    .ok_or_else(|| eyre!("To stack {} does not exist", step.to_stack))?
            } else if step.to_stack > step.from_stack {
                after_from.get_mut(step.to_stack - step.from_stack - 1)
                    .ok_or_else(|| eyre!("To stack {} does not exist", step.to_stack))?
            } else {
                bail!("From stack and to stack are both {}", step.from_stack)
            };

        /*
        println!("moving {} from {} to {}", step.crate_count, step.from_stack, step.to_stack);
        println!("Stack {}: {:?}", step.from_stack, from_stack);
        println!("Stack {}: {:?}", step.to_stack, to_stack);
        */

        match mode {
            Mode::Part1 =>
                from_stack
                .drain((from_stack.len()-step.crate_count)..)
                .rev()
                .for_each(|c| to_stack.push(c)),
            Mode::Part2 =>
                from_stack
                .drain((from_stack.len()-step.crate_count)..)
                .for_each(|c| to_stack.push(c))
            };

        Ok(())
    }
}

pub struct Step {
    from_stack: usize,
    to_stack: usize,
    crate_count: usize
}

mod parsing {
    use crate::*;
    use nom::{
        IResult,
        Parser,
        bytes::complete::tag,
        combinator::into,
        sequence::{preceded, delimited, terminated, pair, tuple, separated_pair},
        character::complete::{char, anychar, line_ending, u8, satisfy},
        branch::alt,
        multi::{many0_count, separated_list1, many1, many0},
    };
    
    #[repr(transparent)]
    struct Layer {
        items: Vec<Option<Crate>>
    }

    impl Layer {
        fn new(items: Vec<Option<Crate>>) -> Self {
            Self { items }
        }
    }

    fn empty_space(input: &str) -> IResult<&str, Option<Crate>> {
        tag("   ").map(|_| None).parse(input)
    }

    fn acrate(input: &str) -> IResult<&str, Option<Crate>> {
        delimited(char('['), anychar, char(']')).map(|l| Some(Crate::new(l))).parse(input)
    }

    fn layeritem(input: &str) -> IResult<&str, Option<Crate>> {
        alt((empty_space, acrate))(input)
    }

    fn layer(input: &str) -> IResult<&str, Layer> {
        terminated(separated_list1(tag(" "), layeritem), line_ending).map(Layer::new).parse(input)
    }

    fn stack_bottom(input: &str) -> IResult<&str, ()> {
        delimited(char(' '), satisfy(|c| c.is_ascii_digit()), char(' ')).map(|_| ()).parse(input)
    }

    fn floor(input: &str) -> IResult<&str, usize> {
        terminated(preceded(stack_bottom, many0_count(preceded(char(' '), stack_bottom))), line_ending).map(|n| n+1).parse(input)
    }

    fn initial_state(input: &str) -> IResult<&str, Ship> {
        pair(many1(layer), floor).map(|(layers, stack_count)| {
            let stacks = vec![vec![]; stack_count];
            let mut ship = Ship { stacks };
            layers.iter()
                .rev()
                .for_each(|layer|
                    layer.items.iter()
                    .enumerate()
                    .filter_map(|(ix, item)|
                        item.map(|c| (ix, c)))
                    .for_each(|(ix, c)| ship.stacks[ix].push(c)));
                    ship
        }).parse(input)
    }

    fn step(input: &str) -> IResult<&str, Step> {
        terminated(tuple((
                (preceded(tag("move "), u8)),
                (preceded(tag(" from "), u8)),
                (preceded(tag(" to "), u8))
        )), line_ending).map(|(crate_count, from_stack, to_stack)| Step{crate_count:crate_count.into(), from_stack:from_stack.into(), to_stack:to_stack.into()}).parse(input)
    }

    pub fn problem(input: &str) -> IResult<&str, (Ship, Vec<Step>)> {
        separated_pair(initial_state, line_ending, many0(step))(input)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let input = std::io::read_to_string(cli.input.get_input()?)?;
    let input_str = input.as_str();
    let (_, (mut ship, steps)) = parsing::problem(input_str).map_err(|e| e.to_owned())?;

    let mut extra_ship = ship.clone();

    steps.iter().map(|step| ship.execute(step, Mode::Part1)).try_collect()?;

    print!("Part1 answer: ");
    ship.stacks.iter().for_each(|stack| print!("{}", stack.last().map(|c| Crate::label(*c)).unwrap_or(' ')));
    println!();

    steps.iter().map(|step| extra_ship.execute(step, Mode::Part2)).try_collect()?;

    print!("Part2 answer: ");
    extra_ship.stacks.iter().for_each(|stack| print!("{}", stack.last().map(|c| Crate::label(*c)).unwrap_or(' ')));
    println!();

    Ok(())
}
