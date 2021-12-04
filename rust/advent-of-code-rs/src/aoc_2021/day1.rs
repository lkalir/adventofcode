use crate::Solution;
use crate::SolutionType;
use advent_of_code_data_rs::get_input;

use itertools::Itertools;

pub struct Day1 {}

impl Solution for Day1 {
    fn get_name() -> &'static str {
        "2021 Day 1: Sonar Sweep"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::One,
        );
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut last = u32::MAX;
        let ret = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .fold(0, |acc, line| {
                let ret = if line > last { acc + 1 } else { acc };
                last = line;
                ret
            });
        SolutionType::Uint(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut last = u32::MAX;
        let ret = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .tuple_windows::<(_, _, _)>()
            .fold(0, |acc, depth| {
                let d = depth.0 + depth.1 + depth.2;
                let ret = if d > last { acc + 1 } else { acc };
                last = d;
                ret
            });
        SolutionType::Uint(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day1::get_input();
        let ret = Day1::part_1(input);
        assert_eq!(ret, SolutionType::Uint(1139));
    }

    #[test]
    fn part_2() {
        let input = Day1::get_input();
        let ret = Day1::part_2(input);
        assert_eq!(ret, SolutionType::Uint(1103));
    }
}
