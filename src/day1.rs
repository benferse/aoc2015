//! Day 1 - Not Quite Lisp

// Maps a set of instructions to a set of matching offsets
fn map_floors(instructions: &str) -> impl Iterator<Item = i32> + '_ {
    instructions.trim().chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        err => panic!("Don't know what to do with a {err}"),
    })
}

/// Given a set of up/down instructions, figure out what floor we
/// end up on
///
/// Examples
///
/// ```
/// # use aoc2015::day1::find_floor;
/// assert_eq!(find_floor("(())"), 0);
/// assert_eq!(find_floor("()()"), 0);
/// assert_eq!(find_floor("((("), 3);
/// assert_eq!(find_floor(")))"), -3);
/// assert_eq!(find_floor("))((((("), 3);
/// ```
pub fn find_floor(instructions: &str) -> i32 {
    map_floors(instructions).sum()
}

/// Given a set of instructions, find the index of the first
/// instruction that causes us to visit the basement (floor -1)
///
/// Examples
///
/// ```
/// # use aoc2015::day1::find_basement;
/// assert_eq!(find_basement(")"), 1);
/// ```
pub fn find_basement(instructions: &str) -> usize {
    map_floors(instructions)
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate()
        .find(|x| x.1 == -1)
        .expect("We should have hit the basement once according to the problem")
        .0
        + 1
}

#[cfg(test)]
mod answers {
    use super::*;

    static INPUT: &str = include_str!("./input/day1.txt");

    #[test]
    pub fn problem1() {
        assert_eq!(find_floor(INPUT), 74);
    }

    #[test]
    pub fn problem2() {
        assert_eq!(find_basement(INPUT), 1795);
    }
}
