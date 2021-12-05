use crate::{SolutionType, SolutionYear};
use advent_of_code_data_rs::{get_input, Day, Year};

pub struct Aoc2016;

impl SolutionYear for Aoc2016 {
    fn get_sln(day: Day) -> Option<(SolutionType, SolutionType)> {
        let input = get_input(Year::TwentySixteen, day)?;
        let _input = &input[..input.len() - 1];

        None
    }
}
