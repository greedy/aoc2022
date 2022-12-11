use aoc2022::prelude::*;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    input: InputCLI<11>
}

trait SliceExt {
    type Elem;

    fn split_around(&self, n: usize) -> Option<(&Self, &Self::Elem, &Self)>;
    fn split_around_mut(&mut self, n: usize) -> Option<(&mut Self, &mut Self::Elem, &mut Self)>;

    /*
    fn iter_around(&self) -> IterAroundSlice<'_, Self::Elem>;
    fn iter_around_mut(&mut self) -> IterAroundMutSlice<'_, Self::Elem>;
    */
}

impl<T> SliceExt for [T] {
    type Elem = T;

    fn split_around(&self, n: usize) -> Option<(&Self, &Self::Elem, &Self)> {
        let (front, mid) = self.split_at(n);
        let (item, back) = mid.split_first()?;
        Some((front, item, back))
    }

    fn split_around_mut(&mut self, n: usize) -> Option<(&mut Self, &mut Self::Elem, &mut Self)> {
        let (front, mid) = self.split_at_mut(n);
        let (item, back) = mid.split_first_mut()?;
        Some((front, item, back))
    }

    /*
    fn iter_around(&self) -> IterAroundSlice<'_, T> {
        IterAroundSlice::new(self)
    }

    fn iter_around_mut(&mut self) -> IterAroundMutSlice<T> {
        IterAroundMutSlice::new(self)
    }
    */
}

/*
struct IterAroundSlice<'a, T> {
    slice: &'a [T],
    front: usize,
    back: usize
}

impl<'a, T> IterAroundSlice<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        Self { slice, front: 0, back: slice.len() }
    }
}

impl<'a, T> Iterator for IterAroundSlice<'a, T> {
    type Item = (&'a [T], &'a T, &'a [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back { return None }
        let splits = self.slice.split_around(self.front).unwrap();
        self.front += 1;
        Some(splits)
    }
}

struct IterAroundMutSlice<'a, T> {
    slice: &'a mut [T],
    front: usize,
    back: usize,
    cur_splits: Option<(&'a mut [T], &'a mut T, &'a mut [T])>
}

impl<'a, T> IterAroundMutSlice<'a, T> {
    fn new(slice: &'a mut [T]) -> Self {
        let back = slice.len();
        Self { slice, front: 0, back, cur_splits: None }
    }
}

impl<'a, T> Iterator for IterAroundMutSlice<'a, T> where Self: 'a {
    type Item = &'a (&'a mut [T], &'a mut T, &'a mut [T]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back { return None }
        self.cur_splits = self.slice.split_around_mut(self.front);
        self.front += 1;
        Some((&self.cur_splits).unwrap())
    }
}
*/

struct Item {
    worry: i64
}

enum Expression {
    OldValue,
    Constant(i64),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>)
}

impl From<i64> for Expression {
    fn from(c: i64) -> Self {
        Self::Constant(c)
    }
}

impl From<i64> for Box<Expression> {
    fn from(c: i64) -> Self {
        Box::new(Expression::Constant(c))
    }
}

enum Test {
    DivisibleBy(i64)
}

impl Item {
    pub fn new(worry: i64) -> Self {
        Self { worry }
    }

    pub fn eval_expression(&self, expr: &Expression) -> i64 {
        match expr {
            Expression::OldValue => self.worry,
            Expression::Constant(x) => *x,
            Expression::Add(l, r) => self.eval_expression(l) + self.eval_expression(r),
            Expression::Mul(l, r) => self.eval_expression(l) * self.eval_expression(r),
        }
    }

    pub fn update_worry(&mut self, expr: &Expression) {
        self.worry = self.eval_expression(expr);
    }

    pub fn eval_test(&self, test: &Test) -> bool {
        match test {
            Test::DivisibleBy(q) => self.worry % q == 0
        }
    }
}

struct Monkey {
    id: usize,
    held_items: Vec<Item>,
    inspect_operation: Expression,
    test: Test,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

struct Throw {
    item: Item,
    to_monkey: usize,
}

impl Monkey {

    fn new<const N:usize>(id: usize, held_items: [i64; N], inspect_operation: Expression, divby: i64, true_monkey: usize, false_monkey: usize) -> Self {
        let held_items = held_items.iter().map(|w| Item::new(*w)).collect();
        let test = Test::DivisibleBy(divby);
        Self { id, held_items, inspect_operation, test, true_monkey, false_monkey, inspection_count: 0 }
    }

    fn take_a_turn(&mut self) -> Vec<Throw> {
        println!("Monkey {}'s turn", self.id);
        self.held_items.drain(0..).enumerate()
            .inspect(|(i,_)| println!("  Inspecting item {}", i))
            .map(|(_,item)| item)
            .inspect(|item| println!("      Worry was {}", item.worry))
            .update(|item| item.update_worry(&self.inspect_operation))
            .inspect(|item| println!("      Worry is now {}", item.worry))
            .inspect(|_| self.inspection_count += 1)
            .update(|item| item.worry /= 3)
            .inspect(|item| println!("      Phew, worry reduced to {}", item.worry))
            .map(|item| {
                let test_result = item.eval_test(&self.test);
                let to_monkey = if test_result { self.true_monkey } else { self.false_monkey };
                println!("    Test was {}, throwing to {}", test_result, to_monkey);
                Throw { item, to_monkey }
            })
            .collect()
    }

    fn catch(&mut self, throw: Throw) {
        assert!(self.id == throw.to_monkey, "id: {}, thrown to: {}", self.id, throw.to_monkey);
        println!("  Monkey {} catches item with worry {}", self.id, throw.item.worry);
        self.held_items.push(throw.item)
    }
}

struct Keepaway {
    monkeys: Vec<Monkey>
}

impl Keepaway {

    pub fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            println!("Round {}", i);
            let (before, cur, after) = self.monkeys.split_around_mut(i).unwrap();
            for throw in cur.take_a_turn() {
                if throw.to_monkey < i {
                    before[throw.to_monkey].catch(throw);
                } else {
                    assert!(throw.to_monkey > i);
                    after[throw.to_monkey - i - 1].catch(throw);
                }
            }
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let _cli = Cli::parse();

    use Expression::*;
    use Test::*;

    let monkeys = vec![
        Monkey::new(0, [54, 98, 50, 94, 69, 62, 53, 85], Mul(OldValue.into(), Constant(13).into()), 3, 2, 1),
        Monkey::new(1, [71, 55, 82], Add(OldValue.into(), 2.into()), 13, 7, 2),
        Monkey::new(2, [77, 73, 86, 72, 87], Add(OldValue.into(), 8.into()), 19, 4, 7),
        Monkey::new(3, [97, 91], Add(OldValue.into(), 1.into()), 17, 6, 5),
        Monkey::new(4, [78, 97, 51, 85, 66, 63, 62], Mul(OldValue.into(), 17.into()), 5, 6, 3),
        Monkey::new(5, [88], Add(OldValue.into(), 3.into()), 7, 1, 0),
        Monkey::new(6, [87, 57, 63, 86, 87, 53], Mul(OldValue.into(), OldValue.into()), 11, 5, 0),
        Monkey::new(7, [73, 59, 82, 65], Add(OldValue.into(), 6.into()), 2, 4, 3),
    ];

    let mut game = Keepaway { monkeys };
    
    for _ in 0..20 {
        game.round();
    }

    let answer : isize = game.monkeys.iter().map(|m| -(m.inspection_count as isize)).k_smallest(2).product();

    println!("part 1 answer is {}", answer);

    Ok(())
}
