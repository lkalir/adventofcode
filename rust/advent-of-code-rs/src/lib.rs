pub mod aoc_2015;
pub mod aoc_2016;
pub mod aoc_2017;
pub mod aoc_2018;
pub mod aoc_2019;
pub mod aoc_2020;
pub mod aoc_2021;
pub(crate) mod macros;
mod traits;
pub(crate) mod utils;

pub use advent_of_code_data_rs::{get_input, Day, Year};
pub use traits::{Solution, SolutionType, SolutionYear};

use aoc_2015::Aoc2015;
use aoc_2016::Aoc2016;
use aoc_2017::Aoc2017;
use aoc_2018::Aoc2018;
use aoc_2019::Aoc2019;
use aoc_2020::Aoc2020;
use aoc_2021::Aoc2021;

pub fn get_sln(day: Day, year: Year) -> Option<(SolutionType, SolutionType)> {
    match year {
        Year::TwentyFifteen => Aoc2015::get_sln(day),
        Year::TwentySixteen => Aoc2016::get_sln(day),
        Year::TwentySeventeen => Aoc2017::get_sln(day),
        Year::TwentyEighteen => Aoc2018::get_sln(day),
        Year::TwentyNineteen => Aoc2019::get_sln(day),
        Year::TwentyTwenty => Aoc2020::get_sln(day),
        Year::TwentyTwentyOne => Aoc2021::get_sln(day),
    }
}
