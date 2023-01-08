use nom::branch::alt;
use nom::bytes::complete::{tag, take_until, take_while};
use nom::character::complete::{alphanumeric1, digit1, newline};
use nom::character::{is_alphabetic, is_alphanumeric, is_space};
use nom::combinator::{map_res, value};
use nom::error::{ErrorKind, ParseError};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;
use std::fs;
use std::ops;

fn main() {
    let contents =
        fs::read_to_string("../input/day11.txt").expect("Should have been able to open te file");
    //println!("{contents:?}");
    let (_, mnk) = Monkey::parse(&contents).unwrap();
    dbg!(mnk);

    // TODO: parse multiple monkeys

    // TODO: inspect

    // TODO: monkey turn

    // TODO: round
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Operand {
    Old,
    Num(u32),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    operand: Operand,
    test_div: u32,
    true_throw: usize,
    false_throw: usize,
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

        Ok((
            rest,
            Self {
                items,
                operation,
                operand,
                test_div,
                true_throw,
                false_throw,
            },
        ))
    }

    //fn inspect(&self, item: u32) -> usize {}
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
