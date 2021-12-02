pub mod aoc_2015;
pub mod aoc_2021;
mod traits;

pub use advent_of_code_data_rs::{get_input, Day, Year};
pub use traits::*;

pub fn get_sln(day: Day, year: Year) -> (SolutionType, SolutionType) {
    let input = get_input(year, day);
    let input = &input[..input.len() - 1];

    match (year, day) {
        (Year::TwentyFifteen, Day::One) => {
            (aoc_2015::Day1::part_1(input), aoc_2015::Day1::part_2(input))
        }
        (Year::TwentyFifteen, Day::Two) => {
            (aoc_2015::Day2::part_1(input), aoc_2015::Day2::part_2(input))
        }
        (Year::TwentyTwentyOne, Day::One) => {
            (aoc_2021::Day1::part_1(input), aoc_2021::Day1::part_2(input))
        }
        _ => todo!(),
    }
}
