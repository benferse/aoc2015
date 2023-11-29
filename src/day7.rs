//! Day 7 - Some Assembly Required

use std::collections::HashMap;

pub struct Circuit<'a> {
    instructions: HashMap<&'a str, Vec<&'a str>>,
    memo: HashMap<&'a str, u16>,
}

impl<'a> Circuit<'a> {
    pub fn load(input: &[&'a str]) -> Circuit<'a> {
        let instructions = input.iter()
            .map(|line| line.split_once("->").expect("Should have had an arrow delimiter"))
            .map(|(spec, wire)| (wire.trim(), spec.split_whitespace().collect()))
            .collect();

        Self { instructions, memo: HashMap::new() }
    }

    pub fn reset(&mut self) {
        self.memo.clear();
    }

    pub fn solve(&mut self, token: &'a str, signal: u16) -> u16 {
        self.memo.insert(token, signal);
        signal
    }

    pub fn evaluate(&mut self, token: &'a str) -> u16 {
        match token.parse() {
            Ok(signal) => signal,
            Err(_) => {
                match self.memo.get(token) {
                    Some(signal) => *signal,
                    None => {
                        let signal = self.execute(token);
                        self.solve(token, signal)
                    }
                }
            }
        }
    }

    fn execute(&mut self, wire: &'a str) -> u16 {
        let steps = self.instructions.get(wire).expect("No instructions for wire {wire}").clone();
        match steps.len() {
            1 => self.evaluate(steps[0]),
            2 => {
                // NOT x
                if steps[0] != "NOT" {
                    unimplemented!("The only unary operator we know is NOT");
                }

                !self.evaluate(steps[1])
            },
            3 => {
                // A binary operator. Both operands may be a signal constant
                // or a reference to another wire
                let lhs = self.evaluate(steps[0]);
                let rhs = self.evaluate(steps[2]);
                match steps[1] {
                    "AND" => lhs & rhs,
                    "OR" => lhs | rhs,
                    "LSHIFT" => lhs << rhs,
                    "RSHIFT" => lhs >> rhs,
                    err => unimplemented!("Binary operand {err} is not supported"),
                }
            },
            _ => unimplemented!("Unknown instructions: {steps:?}"),
        }
    }
}

#[cfg(test)]
mod answers {
    use super::*;
    use test_case::test_case;

    #[test_case(SAMPLE_INPUT, "x" => 123; "sample data x")]
    #[test_case(SAMPLE_INPUT, "y" => 456; "sample data y")]
    #[test_case(SAMPLE_INPUT, "d" => 72; "sample data d")]
    #[test_case(SAMPLE_INPUT, "e" => 507; "sample data e")]
    #[test_case(SAMPLE_INPUT, "f" => 492; "sample data f")]
    #[test_case(SAMPLE_INPUT, "g" => 114; "sample data g")]
    #[test_case(SAMPLE_INPUT, "h" => 65412; "sample data h")]
    #[test_case(SAMPLE_INPUT, "i" => 65079; "sample data i")]
    #[test_case(personal_input().as_slice(), "a" => 16076; "problem 1 data")]
    pub fn problem1(input: &[&str], target: &str) -> u16 {
        let mut circuit = Circuit::load(input);
        circuit.evaluate(target)
    }

    #[test_case(personal_input().as_slice(), "a" => 2797; "problem 2 data")]
    pub fn problem2(input: &[&str], target: &str) -> u16 {
        let mut circuit = Circuit::load(input);
        let a = circuit.evaluate(target);
        circuit.reset();
        circuit.solve("b", a);

        circuit.evaluate(target)
    }

    fn personal_input() -> Vec<&'static str> {
        include_str!("./input/day7.txt")
            .lines()
            .map(str::trim)
            .collect()
    }

    const SAMPLE_INPUT: &[&str] = &[
        "123 -> x",
        "456 -> y",
        "x AND y -> d",
        "x OR y -> e",
        "x LSHIFT 2 -> f",
        "y RSHIFT 2 -> g",
        "NOT x -> h",
        "NOT y -> i",
    ];
}
