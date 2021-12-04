use std::hint::unreachable_unchecked;

use crate::Solution;
use crate::SolutionType;
use advent_of_code_data_rs::get_input;

pub struct Day1 {}

impl Solution for Day1 {
    fn get_name() -> &'static str {
        "2015 Day 1: Not Quite Lisp"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyFifteen,
            advent_of_code_data_rs::Day::One,
        );
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let cnt = input.chars().filter(|&c| c == '(').count();
        SolutionType::Int(2 * cnt as i32 - input.len() as i32)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut floor = 0;

        for (idx, c) in input.chars().enumerate() {
            match c {
                '(' => floor += 1,
                ')' => floor -= 1,
                _ => unsafe { unreachable_unchecked() },
            }

            if floor < 0 {
                return SolutionType::Uint(idx as u32 + 1);
            }
        }

        SolutionType::Uint(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day1::get_input();
        let ret = Day1::part_1(input);
        assert_eq!(ret, SolutionType::Int(138));
    }

    #[test]
    fn part_2() {
        let input = Day1::get_input();
        let ret = Day1::part_2(input);
        assert_eq!(ret, SolutionType::Uint(1771));
    }
}
