//! Day 6 - Probably a Fire Hazard

use crate::prelude::*;
use nom::{IResult, error::context, branch::alt, bytes::complete::tag, sequence::separated_pair};
use nom::character::complete::{char, digit1};
use nom::sequence::tuple;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "turn on" => Self::TurnOn,
            "turn off" => Self::TurnOff,
            "toggle" => Self::Toggle,
            err => unimplemented!("Cannot handle operation: {err}"),
        }
    }
}

impl Operation {
    /// Parse an operation from tagged text
    ///
    /// ```
    /// # use aoc2015::day6::*;
    /// assert_eq!(Operation::parse("turn on"), Ok(("", Operation::TurnOn)));
    /// assert_eq!(Operation::parse("turn off xxx"), Ok((" xxx", Operation::TurnOff)));
    /// assert_eq!(Operation::parse("toggle xxx"), Ok((" xxx", Operation::Toggle)));
    /// ```
    pub fn parse(input: &str) -> IResult<&str, Self> {
        context(
            "operation",
            alt((tag("turn on"), tag("turn off"), tag("toggle")))
        )(input)
        .map(|(next, res)| (next, res.into()))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Parse a point from tagged text
    ///
    /// ```
    /// # use aoc2015::day6::*;
    /// assert_eq!(Point::parse("123,456"), Ok(("", Point { x: 123, y: 456 })));
    /// assert!(Point::parse("123,abd").is_err());
    /// ```
    pub fn parse(input: &str) -> IResult<&str, Self> {
        context(
            "point",
            separated_pair(digit1, char(','), digit1)
        )(input)
        .map(|(next, res)| (next, res.into()))
    }
}

impl From<(&str, &str)> for Point {
    fn from(value: (&str, &str)) -> Self {
        Self {
            x: value.0.parse().expect("Should have been a number"),
            y: value.1.parse().expect("Should have been a number"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub operation: Operation,
    pub start_point: Point,
    pub end_point: Point,
}

impl Instruction {
    /// Parse an entire instruction
    ///
    /// ```
    /// # use aoc2015::day6::*;
    /// let i = Instruction::parse("turn on 1,1 through 2,2\n");
    /// assert_eq!(i, Instruction {
    ///     operation: Operation::TurnOn,
    ///     start_point: Point { x: 1, y: 1 },
    ///     end_point: Point { x: 2, y: 2 }
    /// });
    /// ```
    pub fn parse(input: &str) -> Self {
        let (next, res) = context(
            "instruction",
            tuple((
                ws(Operation::parse),
                ws(Point::parse),
                ws(tag("through")),
                ws(Point::parse),
            ))
        )(input).expect("Instruction should have parsed correctly");

        if !next.is_empty() {
            unimplemented!("Extra tokens in the input isn't handled");
        }

        let (operation, start_point, _, end_point) = res;
        Instruction { operation, start_point, end_point }
    }

}

#[cfg(test)]
mod answers {
    use super::*;
    use std::collections::HashSet;
    use std::collections::HashMap;

    static INPUT: &str = include_str!("./input/day6.txt");

    #[test]
    pub fn problem1() {
        let grid = INPUT.lines()
            .map(Instruction::parse)
            .fold(HashSet::new(), |mut grid, i| {
                for x in i.start_point.x..=i.end_point.x {
                    for y in i.start_point.y..=i.end_point.y {
                        let pos = Point { x, y };
                        match i.operation {
                            Operation::TurnOn => { grid.insert(pos); },
                            Operation::TurnOff => { grid.remove(&pos); },
                            Operation::Toggle => {
                                if !grid.remove(&pos) {
                                    grid.insert(pos);
                                }
                            },
                        }
                    }
                }

                grid
            });

        assert_eq!(grid.len(), 400410);
    }

    #[test]
    pub fn problem2() {
        let grid: HashMap<Point, u32> = INPUT.lines()
            .map(Instruction::parse)
            .fold(HashMap::new(), |mut grid, i| {
                for x in i.start_point.x..=i.end_point.x {
                    for y in i.start_point.y..=i.end_point.y {
                        let pos = Point { x, y };
                        let entry = grid.entry(pos).or_insert(0);
                        match i.operation {
                            Operation::TurnOn => *entry += 1,
                            Operation::TurnOff => *entry = entry.saturating_sub(1),
                            Operation::Toggle => *entry += 2, }
                    }
                }

                grid
            });

        assert_eq!(grid.values().sum::<u32>(), 15343601);
    }
}
