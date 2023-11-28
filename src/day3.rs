//! Day 3 - Perfectly Spherical Houses in a Vacuum

use std::collections::HashMap;

/// Given a set of instructions from elf HQ, returns the table
/// of visits to each location and the number of stops that we
/// made
///
/// ```
/// # use aoc2015::day3::*;
/// let visits = collect_visits(">".chars());
/// assert_eq!(visits.len(), 2);
/// ```
pub fn collect_visits(moves: impl Iterator<Item=char>) -> HashMap<(isize, isize), u32> {
    let mut visits = HashMap::new();
    let start = (0, 0);
    visits.insert(start, 1);

    moves
        .fold(start, |mut loc, next| {
            match next {
                '<' => loc = (loc.0 - 1, loc.1),
                '>' => loc = (loc.0 + 1, loc.1),
                '^' => loc = (loc.0, loc.1 + 1),
                'v' => loc = (loc.0, loc.1 - 1),
                err => panic!("Don't know what to do with a {err}"),
            }

            *visits.entry(loc).or_insert(0) += 1;
            loc
        });

    visits
}

#[cfg(test)]
mod answers {
    use super::*;
    use std::collections::HashSet;

    static INPUT: &str = include_str!("./input/day3.txt");

    #[test]
    pub fn problem1() {
        let visits = collect_visits(INPUT.trim().chars());
        assert_eq!(visits.len(), 2572);
    }

    #[test]
    pub fn problem2() {
        let santa_moves = INPUT.trim().chars().step_by(2);
        let robot_moves = INPUT.trim().chars().skip(1).step_by(2);

        let santa_visits = collect_visits(santa_moves);
        let robot_visits = collect_visits(robot_moves);

        let mut all_houses = HashSet::<(isize, isize)>::new();
        santa_visits.keys().collect_into(&mut all_houses);
        robot_visits.keys().collect_into(&mut all_houses);

        assert_eq!(all_houses.len(), 2631);
    }
}
