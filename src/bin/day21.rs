use std::borrow::Borrow;
use std::fmt::Debug;
use std::str::FromStr;

use aoc2022::prelude::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::cell::Cell;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<21>
}

#[derive(Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl FromStr for Op {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Subtract),
            "*" => Ok(Op::Multiply),
            "/" => Ok(Op::Divide),
            _ => Err(eyre!("Expected +, -, *, or /"))
        }
    }
}

#[derive(Debug)]
enum Expression {
    Const(i64),
    Binary { lhs: String, op: Op, rhs: String }
}

impl FromStr for Expression {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect_vec();
        if parts.len() == 1 {
            Ok(Expression::Const(parts[0].parse()?))
        } else if parts.len() == 3 {
            let lhs = parts[0].to_owned();
            let rhs = parts[2].to_owned();
            let op = parts[1].parse()?;
            Ok(Expression::Binary { lhs, op, rhs })
        } else {
            Err(eyre!("Invalid expression"))
        }
    }
}

struct Monkey<'a> {
    name: String,
    expr: Expression,
    waiters: Cell<Vec<&'a Monkey<'a>>>,
    lhs_value: Cell<Option<i64>>,
    rhs_value: Cell<Option<i64>>,
    yelled: Cell<Option<i64>>,
}

impl<'a> Debug for Monkey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("name", &self.name)
            .field("expr", &self.expr)
            .field("waiters", &{
                let waiters = self.waiters.take();
                let names = waiters.iter().map(|m| m.name.as_str()).collect_vec();
                self.waiters.set(waiters);
                names
            })
        .field("lhs_value", &self.lhs_value)
            .field("rhs_value", &self.rhs_value)
            .field("yelled", &self.yelled)
            .finish()
    }
}


impl<'a> Monkey<'a> {
    fn new(name: String, expr: Expression) -> Self {
        let waiters = Cell::new(vec![]);
        let lhs_value = Cell::new(None);
        let rhs_value = Cell::new(None);
        let yelled = Cell::new(None);
        Self { name, expr, waiters, lhs_value, rhs_value, yelled }
    }

    fn yell(&'a self) {
        let value = match &self.expr {
            Expression::Const(v) => *v,
            Expression::Binary { lhs:_, op, rhs:_ } => {
                let lhs = self.lhs_value.get().expect("hear lhs before yell");
                let rhs = self.rhs_value.get().expect("hear rhs before yell");
                match op {
                    Op::Add => lhs + rhs,
                    Op::Subtract => lhs - rhs,
                    Op::Multiply => lhs * rhs,
                    Op::Divide => lhs / rhs
                }
            }
        };
        self.yelled.set(Some(value));
        for listener in self.waiters.take() {
            listener.hear(self, value);
        }
    }

    fn hear(&'a self, yeller: &'a Monkey, value: i64) {
        match &self.expr {
            Expression::Const(_) => (),
            Expression::Binary { lhs, op, rhs } => {
                if yeller.name == *lhs {
                    self.lhs_value.set(Some(value));
                }
                if yeller.name == *rhs {
                    self.rhs_value.set(Some(value));
                }
                if self.lhs_value.get().is_some() && self.rhs_value.get().is_some() {
                    self.yell()
                }
            }
        }
    }
}

impl<'a> FromStr for Monkey<'a> {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, expr) = s.split(": ").collect_tuple().ok_or_else(|| eyre!("Expected '<name>: <expression>'"))?;
        let name = name.to_owned();
        let expr = expr.parse()?;
        Ok(Monkey::new(name, expr))
    }
}

fn solve(monkeys: &HashMap<String, &mut Monkey>, monkey: &Monkey, target: i64) -> Result<i64> {
    dbg!(monkey);
    if monkey.name == "humn" {
        Ok(target)
    } else {
        match &monkey.expr {
            Expression::Const(_) => bail!("solving a const!"),
            Expression::Binary { lhs, op, rhs } => {
                if let Some(lhs_value) = monkey.lhs_value.get() {
                    let rhs = monkeys.get(rhs).expect("rhs to exist");
                    match op {
                        Op::Add => solve(monkeys, rhs, target - lhs_value),
                        // lhs - rhs = target ==> lhs = rhs + target ==> lhs - target = rhs
                        Op::Subtract => solve(monkeys, rhs, lhs_value - target),
                        Op::Multiply => solve(monkeys, rhs, target / lhs_value),
                        // lhs / rhs = target ==> lhs = target * rhs ==> lhs / target = rhs
                        Op::Divide => solve(monkeys, rhs, lhs_value / target)
                    }
                } else if let Some(rhs_value) = monkey.rhs_value.get() {
                    let lhs = monkeys.get(lhs).expect("rhs to exist");
                    match op {
                        Op::Add => solve(monkeys, lhs, target - rhs_value),
                        // lhs - rhs = target ==> lhs = target + rhs
                        Op::Subtract => solve(monkeys, lhs, target + rhs_value),
                        Op::Multiply => solve(monkeys, lhs, target / rhs_value),
                        // lhs / rhs = target ==> lhs = target * rhs 
                        Op::Divide => solve(monkeys, lhs, target * rhs_value)
                    }
                } else {
                    bail!("neither side had yelled");
                }
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let monkeys: typed_arena::Arena<Monkey> = typed_arena::Arena::new();
    let mut monkeys_by_name = HashMap::new();

    for line in cli.input.get_input()?.lines() {
        let line = line?;

        let monkey = monkeys.alloc(line.parse()?);
        monkeys_by_name.insert(monkey.name.to_owned(), monkey);
    }

    for monkey in monkeys_by_name.values() {
        match &monkey.expr {
            Expression::Const(_) => (),
            Expression::Binary { lhs, op: _, rhs } => {
                let lhs_monkey = monkeys_by_name.get(lhs).expect("referenced monkey to exist");
                let mut waiters = lhs_monkey.waiters.take();
                waiters.push(monkey);
                lhs_monkey.waiters.set(waiters);

                let rhs_monkey = monkeys_by_name.get(rhs).expect("referenced monkey to exist");
                let mut waiters = rhs_monkey.waiters.take();
                waiters.push(monkey);
                rhs_monkey.waiters.set(waiters);
            }
        }
    }

    for monkey in monkeys_by_name.values() {
        if matches!(monkey.expr, Expression::Const(_)) && monkey.name != "humn" {
            monkey.yell();
        }
    }

    let root = monkeys_by_name.get("root").expect("a monkey named root");

    let (root_lhs, root_rhs) = match &root.expr {
        Expression::Const(_) => bail!("root monkey should be a binary op"),
        Expression::Binary { lhs, op:_, rhs } => {
            (monkeys_by_name.get(lhs).expect("root's lhs to exist"),
            monkeys_by_name.get(rhs).expect("root's rhs to exist"))
        }
    };

    let solution = if root_lhs.yelled.get().is_none() {
        solve(&monkeys_by_name, root_lhs, root_rhs.yelled.get().expect("rhs to have yelled"))
    } else {
        solve(&monkeys_by_name, root_rhs, root_lhs.yelled.get().expect("lhs to have yelled"))
    }?;

    println!("Solution is {}", solution);

    Ok(())
}

