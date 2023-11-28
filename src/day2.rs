//! Day 2 - I Was Told There Would Be No Math

use std::num::ParseIntError;

pub struct Present {
    pub l: u32,
    pub w: u32,
    pub h: u32,
}

impl std::str::FromStr for Present {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s.trim().split('x').collect::<Vec<_>>();
        Ok(Self {
            l: bits[0].parse()?,
            w: bits[1].parse()?,
            h: bits[2].parse()?,
        })
    }
}

impl Present {
    /// Find the total surface area of the present
    ///
    /// Examples
    ///
    /// ```
    /// # use aoc2015::day2::*;
    /// let p1: Present = "2x3x4".parse().unwrap();
    /// assert_eq!(p1.total_area(), 52);
    /// ```
    pub fn total_area(&self) -> u32 {
        2 * (self.l * (self.h + self.w) + (self.h * self.w))
    }

    /// Find the length of ribbon needed for the bow
    ///
    /// ```
    /// # use aoc2015::day2::*;
    /// let p1: Present = "2x3x4".parse().unwrap();
    /// assert_eq!(p1.bow_length(), 24);
    /// ```
    pub fn bow_length(&self) -> u32 {
        self.l * self.h * self.w
    }

    /// Find the length of ribbon needed to wrap the present
    ///
    /// ```
    /// # use aoc2015::day2::*;
    /// let p1: Present = "2x3x4".parse().unwrap();
    /// assert_eq!(p1.ribbon_length(), 10);
    /// ```
    pub fn ribbon_length(&self) -> u32 {
        let mut dims = [ self.l, self.h, self.w ];
        dims.sort();

        dims[0] + dims[0] + dims[1] + dims[1]
    }

    /// Given the dimensions of a present, figure out the total
    /// surface area
    ///
    /// ```
    /// # use aoc2015::day2::*;
    /// let p1: Present = "2x3x4".parse().unwrap();
    /// assert_eq!(p1.slack(), 6);
    /// ```
    pub fn slack(&self) -> u32 {
        [ self.l * self.h, self.l * self.w, self.h * self.w ]
            .iter()
            .min()
            .copied()
            .expect("Shouldn't have been empty")
    }
}


#[cfg(test)]
mod answers {
    use super::*;

    static INPUT: &str = include_str!("./input/day2.txt");

    #[test]
    pub fn problem1() {
        let total = INPUT
            .lines()
            .flat_map(|line| line.parse::<Present>())
            .fold(0, |accum, x| accum + x.total_area() + x.slack());

        assert_eq!(total, 1598415);
    }

    #[test]
    pub fn problem2() {
        let total = INPUT
            .lines()
            .flat_map(|line| line.parse::<Present>())
            .fold(0, |accum, x| accum + x.ribbon_length() + x.bow_length());

        assert_eq!(total, 3812909);
    }
}
