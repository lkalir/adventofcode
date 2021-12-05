use crate::{macros::add_sln, Solution, SolutionType, SolutionYear};
use advent_of_code_data_rs::{get_input, Day, Year};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
pub use day5::Day5;

pub struct Aoc2021;

impl SolutionYear for Aoc2021 {
    fn get_sln(day: Day) -> Option<(SolutionType, SolutionType)> {
        let input = get_input(Year::TwentyTwentyOne, day)?;
        let input = &input[..input.len() - 1];

        match day {
            Day::One => add_sln!(Day1, input),
            Day::Two => add_sln!(Day2, input),
            Day::Three => add_sln!(Day3, input),
            Day::Four => add_sln!(Day4, input),
            Day::Five => add_sln!(Day5, input),
            _ => None,
        }
    }
}
