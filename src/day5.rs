//! Day 5 - Doesn't He Have Intern-Elves For This?

use std::collections::HashMap;

/// Determine if a string has at least three (not necessarily distinct)
/// vowels
///
/// ```
/// # use aoc2015::day5::*;
/// assert!(has_three_vowels("aaa"));
/// assert!(has_three_vowels("aeo"));
/// assert!(has_three_vowels("abcdefghi"));
/// assert!(!has_three_vowels("xyzzy"));
/// ```
pub fn has_three_vowels(s: &str) -> bool {
    s.chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .nth(2)
        .is_some()
}

/// Determine if a string has at least one doubled letter
///
/// ```
/// # use aoc2015::day5::*;
/// assert!(has_doubled_letter("aabcde"));
/// assert!(has_doubled_letter("abbba"));
/// assert!(!has_doubled_letter("abcde"));
/// ```
pub fn has_doubled_letter(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .filter(|&w| w[0] == w[1])
        .count() > 0
}

/// Determine if a string has any of the forbidden substrings
///
/// ```
/// # use aoc2015::day5::*;
/// assert!(has_forbidden_substring("abanana"));
/// assert!(has_forbidden_substring("cdefgh"));
/// assert!(has_forbidden_substring("pqrstuv"));
/// assert!(has_forbidden_substring("tuvwxyz"));
/// assert!(!has_forbidden_substring("ghijklm"));
/// ```
pub fn has_forbidden_substring(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .filter(|&w| {
            (w[0] == b'a' && w[1] == b'b') ||
            (w[0] == b'c' && w[1] == b'd') ||
            (w[0] == b'p' && w[1] == b'q') ||
            (w[0] == b'x' && w[1] == b'y')
        })
        .count() > 0
}

/// Determine if a string has at least one non-overlapping pair
///
/// ```
/// # use aoc2015::day5::*;
/// assert!(has_nonoverlapping_pair("aaaa"));
/// assert!(has_nonoverlapping_pair("qjhvhtzxzqqjkmpb"));
/// assert!(has_nonoverlapping_pair("xxyxx"));
/// assert!(has_nonoverlapping_pair("uurcxstgmygtbstg"));
/// assert!(!has_nonoverlapping_pair("ieodomkazucvgmuy"));
/// ```
pub fn has_nonoverlapping_pair(s: &str) -> bool {
    // Iterate over every window. If we've seen this pair before
    // *and* it wasn't at the position immediately before this one,
    // we're good
    let mut seen_pairs = HashMap::new();

    for (current_position, pair) in s.trim().as_bytes().windows(2).enumerate() {
        let first_position = seen_pairs.entry(pair).or_insert(current_position);

        // If this isn't the first time we've seen it, and the last time we saw it wasn't
        // immediately before this, we're done
        if current_position - *first_position > 1 {
            return true;
        }
    }

    false
}

/// Determine if the string has any "fencepost" letters
/// ```
/// # use aoc2015::day5::*;
/// assert!(has_fenceposts("qjhvhtzxzqqjkmpb"));
/// assert!(has_fenceposts("xxyxx"));
/// assert!(!has_fenceposts("uurcxstgmygtbstg"));
/// assert!(has_fenceposts("ieodomkazucvgmuy"));
/// ```
pub fn has_fenceposts(s: &str) -> bool {
    s.as_bytes()
        .windows(3)
        .filter(|&w| w[0] == w[2])
        .count() > 0
}

/// See if the provided string matches all of the v1 nice vs naughty rules
///
/// ```
/// # use aoc2015::day5::*;
/// assert!(is_nice_v1("ugknbfddgicrmopn"));
/// assert!(is_nice_v1("aaa"));
/// assert!(!is_nice_v1("jchzalrnumimnmhp"));
/// assert!(!is_nice_v1("haegwjzuvuyypxyu"));
/// assert!(!is_nice_v1("dvszwmarrgswjxmb"));
/// ```
pub fn is_nice_v1(s: &str) -> bool {
    has_three_vowels(s) && has_doubled_letter(s) && !has_forbidden_substring(s)
}

/// ```
/// # use aoc2015::day5::*;
/// assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
/// assert!(is_nice_v2("xxyxx"));
/// assert!(!is_nice_v2("uurcxstgmygtbstg"));
/// assert!(!is_nice_v2("ieodomkazucvgmuy"));
/// ```
pub fn is_nice_v2(s: &str) -> bool {
    has_nonoverlapping_pair(s) && has_fenceposts(s)
}

#[cfg(test)]
mod answers {
    use super::*;

    static INPUT: &str = include_str!("./input/day5.txt");

    #[test]
    pub fn problem1() {
        let nice_strings = INPUT
            .lines()
            .filter(|line| is_nice_v1(line))
            .count();

        assert_eq!(nice_strings, 255);
    }

    #[test]
    pub fn problem2() {
        let nice_strings = INPUT
            .lines()
            .filter(|line| is_nice_v2(line))
            .count();

        assert_eq!(nice_strings, 55);
    }
}
