pub mod aoc_2015;
pub mod aoc_2021;
mod traits;
pub(crate) mod utils;

pub use advent_of_code_data_rs::{get_input, Day, Year};
pub use traits::*;

macro_rules! add_sln {
    ($year:ident, $day:ident, $input:expr) => {
        ($year::$day::part_1($input), $year::$day::part_2($input))
    };
}

pub fn get_sln(day: Day, year: Year) -> (SolutionType, SolutionType) {
    let input = get_input(year, day);
    let input = &input[..input.len() - 1];

    match (year, day) {
        (Year::TwentyFifteen, Day::One) => add_sln!(aoc_2015, Day1, input),
        (Year::TwentyFifteen, Day::Two) => add_sln!(aoc_2015, Day2, input),
        (Year::TwentyTwentyOne, Day::One) => add_sln!(aoc_2021, Day1, input),
        (Year::TwentyTwentyOne, Day::Two) => add_sln!(aoc_2021, Day2, input),
        (Year::TwentyTwentyOne, Day::Three) => add_sln!(aoc_2021, Day3, input),
        _ => todo!(),
    }
}
