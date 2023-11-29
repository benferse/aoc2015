//! Advent of Code 2015

#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub mod prelude {
    use nom::IResult;
    use nom::character::complete::multispace0;
    use nom::error::ParseError;
    use nom::sequence::delimited;

    /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and 
    /// trailing whitespace, returning the output of `inner`.
    pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
    {
        delimited(
            multispace0,
            inner,
            multispace0
        )
    }
}
