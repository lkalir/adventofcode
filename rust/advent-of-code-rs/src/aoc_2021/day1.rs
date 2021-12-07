use std::hint::unreachable_unchecked;

use crate::{
    utils::{
        asciibuf::{AsciiBuf, AsciiBuffable},
        circularbuf::CircularBuf,
    },
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;

pub struct Day1;

impl Solution for Day1 {
    fn get_name() -> &'static str {
        "2021 Day 1: Sonar Sweep"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::One,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut last = u32::MAX;
        let ret = input
            .lines()
            .map(|line| match line.len() {
                3 => {
                    let l: &AsciiBuf<3> = line.to_ascii_buf();
                    l.as_dec_ascii()
                }
                4 => {
                    let l: &AsciiBuf<4> = line.to_ascii_buf();
                    l.as_dec_ascii()
                }
                _ => unsafe { unreachable_unchecked() },
            })
            .fold(0, |acc, line| {
                let ret = if line > last { acc + 1 } else { acc };
                last = line;
                ret
            });
        SolutionType::Uint(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut buf = CircularBuf::<u32, 3>::new();
        let mut vals = input.lines().map(|line| match line.len() {
            3 => {
                let l: &AsciiBuf<3> = line.to_ascii_buf();
                l.as_dec_ascii()
            }
            4 => {
                let l: &AsciiBuf<4> = line.to_ascii_buf();
                l.as_dec_ascii()
            }
            _ => unsafe { unreachable_unchecked() },
        });

        for val in vals.by_ref().take(3) {
            buf.insert(val);
        }

        let mut last: u32 = buf.iter().sum();

        let ret = vals.fold(0, |acc, val| {
            let popped = buf.insert(val);
            let depth = last - popped + val;
            let ret = if depth > last { acc + 1 } else { acc };
            last = depth;
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
