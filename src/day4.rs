//! Day 4 - The Ideal Stocking Stuffer

/// Given a secret key, find the lowest positive number
/// that produces an AdventCoin friendly hash
///
/// ```
/// # use aoc2015::day4::*;
/// assert_eq!(mine_advent_coin_v1("abcdef"), 609043);
/// assert_eq!(mine_advent_coin_v1("pqrstuv"), 1048970);
/// ````
pub fn mine_advent_coin_v1(secret_key: &str) -> u32 {
    for idx in 0..=u32::MAX {
        let input = format!("{}{}", secret_key, idx);
        let digest = *md5::compute(input);

        if digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10 {
            return idx;
        }
    }

    panic!("Shit, didn't find one for key {secret_key}");
}

pub fn mine_advent_coin_v2(secret_key: &str) -> u32 {
    for idx in 0..=u32::MAX {
        let input = format!("{}{}", secret_key, idx);
        let digest = *md5::compute(input);

        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return idx;
        }
    }

    panic!("Shit, didn't find one for key {secret_key}");
}

#[cfg(test)]
mod answers {
    use super::*;

    #[test]
    pub fn problem1() {
        assert_eq!(mine_advent_coin_v1("iwrupvqb"), 346386);
    }

    #[test]
    pub fn problem2() {
        assert_eq!(mine_advent_coin_v2("iwrupvqb"), 9958218);
    }
}
