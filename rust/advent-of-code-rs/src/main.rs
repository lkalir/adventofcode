mod aoc_2015;
mod aoc_2016;
mod aoc_2017;
mod aoc_2018;
mod aoc_2019;
mod aoc_2020;

use advent_of_code_rs::{get_sln, Solution, SolutionType};
use clap::{App, Arg};
use num_traits::FromPrimitive;

fn main() {
    let matches = App::new("adventofcode")
        .arg(Arg::with_name("day").required(true).takes_value(true))
        .arg(Arg::with_name("year").required(true).takes_value(true))
        .get_matches();

    if let (Ok(day), Ok(year)) = (
        str::parse(matches.value_of("day").unwrap()),
        str::parse(matches.value_of("year").unwrap()),
    ) {
        if let (Some(day), Some(year)) =
            (FromPrimitive::from_u8(day), FromPrimitive::from_u16(year))
        {
            let (p1, p2) = get_sln(day, year);
            println!("{:?} {:?}", p1, p2);
        }
    }
}
