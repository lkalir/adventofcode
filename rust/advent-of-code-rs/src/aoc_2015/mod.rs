use crate::{macros::add_sln, Solution, SolutionType, SolutionYear};
use advent_of_code_data_rs::{get_input, Day, Year};

mod day1;
mod day2;

pub use day1::Day1;
pub use day2::Day2;

pub struct Aoc2015;

impl SolutionYear for Aoc2015 {
    fn get_sln(day: Day) -> Option<(SolutionType, SolutionType)> {
        let input = get_input(Year::TwentyFifteen, day)?;
        let input = &input[..input.len() - 1];

        match day {
            Day::One => add_sln!(Day1, input),
            Day::Two => add_sln!(Day2, input),
            _ => None,
        }
    }
}
