use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{digit1, newline};
use nom::combinator::{map_res, value};
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("../input/day11.txt").expect("Should have been able to open te file");
    //println!("{contents:?}");
    //let (_, mnk) = Monkey::parse(&contents).unwrap();

    let mut monkeys = Vec::new();

    let (rest, mnk) = Monkey::parse(&contents).unwrap();
    monkeys.push(mnk);
    let (rest, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);
    let (rest, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);
    let (rest, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);
    let (rest, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);
    let (rest, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);
    let (rest, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);
    let (_, mnk) = Monkey::parse(rest.strip_prefix('\n').unwrap()).unwrap();
    monkeys.push(mnk);

    //dbg!(monkeys);

    let num_monkeys = monkeys.len();

    for j in 0..10000 {
        //println!("{j}");
        for i in 0..num_monkeys {
            let output = monkeys[i].inspect();

            for (item, dest) in output {
                monkeys[dest].items.push(item);
            }
        }
    }

    for (i, mnky) in monkeys.iter().enumerate() {
        println!("{i}: {}", mnky.inspect_count);
    }
}

fn parse_items(i: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(", "), map_res(digit1, |s: &str| s.parse::<u32>()))(i)
}

fn parse_items_line(i: &str) -> IResult<&str, Vec<u32>> {
    preceded(tag("  Starting items: "), parse_items)(i)
}

fn parse_operation(i: &str) -> IResult<&str, (Operation, Operand)> {
    separated_pair(
        preceded(
            tag("  Operation: new = old "),
            alt((
                value(Operation::Add, tag("+")),
                value(Operation::Multiply, tag("*")),
            )),
        ),
        tag(" "),
        alt((
            value(Operand::Old, tag("old")),
            map_res(digit1, |x: &str| {
                Ok::<Operand, ErrorKind>(Operand::Num(x.parse::<u32>().unwrap()))
            }),
        )),
    )(i)
}

fn parse_test(i: &str) -> IResult<&str, u32> {
    preceded(
        tag("  Test: divisible by "),
        map_res(digit1, str::parse::<u32>),
    )(i)
}

fn parse_throw(i: &str) -> IResult<&str, usize> {
    map_res(
        preceded(
            take_while(|x: char| x.is_alphabetic() || x.is_whitespace() || x == ':'),
            digit1,
        ),
        str::parse::<usize>,
    )(i)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
enum Operand {
    Old,
    Num(u32),
}

#[derive(Debug, Clone)]
struct Item {
    divisible: HashMap<u32, u32>,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    operand: Operand,
    test_div: u32,
    true_throw: usize,
    false_throw: usize,
    inspect_count: u32,
}

impl Monkey {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (rest, _) =
            tuple::<_, _, (_, ErrorKind), _>((tag("Monkey "), digit1, tag(":"), newline))(i)
                .unwrap();
        let (rest, items) = terminated(parse_items_line, newline)(rest).unwrap();
        let (rest, (operation, operand)) = terminated(parse_operation, newline)(rest).unwrap();
        let (rest, test_div) = terminated(parse_test, newline)(rest).unwrap();
        let (rest, true_throw) = terminated(parse_throw, newline)(rest).unwrap();
        let (rest, false_throw) = terminated(parse_throw, newline)(rest).unwrap();

        let mut real_items: Vec<Item> = Vec::new();

        let list_of_divisors = [17, 7, 13, 2, 19, 3, 5, 11, 23];

        for item in items {
            let mut item_hashmap = HashMap::new();
            for i in list_of_divisors {
                item_hashmap.insert(i, item % i);
            }
            let item_hashmap = Item {
                divisible: item_hashmap,
            };
            real_items.push(item_hashmap);
        }

        Ok((
            rest,
            Self {
                items: real_items,
                operation,
                operand,
                test_div,
                true_throw,
                false_throw,
                inspect_count: 0,
            },
        ))
    }

    fn op(&self, x: &mut Item) {
        let list_of_divisors = [17, 7, 13, 2, 19, 3, 5, 11, 23];

        for key in list_of_divisors {
            let v = x.divisible.get(&key).unwrap();
            match (self.operation, self.operand) {
                (Operation::Add, Operand::Old) => *x.divisible.get_mut(&key).unwrap() *= 2,
                (Operation::Add, Operand::Num(y)) => *x.divisible.get_mut(&key).unwrap() += y,
                (Operation::Multiply, Operand::Old) => *x.divisible.get_mut(&key).unwrap() *= *v,
                (Operation::Multiply, Operand::Num(y)) => *x.divisible.get_mut(&key).unwrap() *= y,
            }

            *x.divisible.get_mut(&key).unwrap() %= key;
        }
    }

    fn test(&self, x: &Item) -> usize {
        if x.divisible.get(&self.test_div).unwrap() == &0 {
            self.true_throw
        } else {
            self.false_throw
        }
    }

    fn inspect(&mut self) -> Vec<(Item, usize)> {
        // inspect items and return list of (item, monkey to throw to)
        let mut output: Vec<(Item, usize)> = Vec::new();

        for item in self.items.iter() {
            let mut itemm = item.clone();
            self.op(&mut itemm);

            let dest = self.test(&itemm);

            output.push((itemm, dest));
        }

        self.inspect_count += self.items.len() as u32;
        // empty items from this monkey after inspecting
        self.items = Vec::new();

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_items() {
        assert_eq!(parse_items("79, 98"), Ok(("", vec![79, 98])));
        assert_eq!(parse_items("79, 98abc"), Ok(("abc", vec![79, 98])));
    }

    #[test]
    fn test_parse_items_line() {
        assert_eq!(
            parse_items_line("  Starting items: 79, 98"),
            Ok(("", vec![79, 98]))
        );
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("  Operation: new = old * 19"),
            Ok(("", (Operation::Multiply, Operand::Num(19))))
        );
    }

    #[test]
    fn test_parse_throw() {
        assert_eq!(parse_throw("    If true: throw to monkey 2"), Ok(("", 2)));
    }
}
