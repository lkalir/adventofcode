use crate::{
    macros::ascii_buf_dec,
    utils::asciibuf::{AsciiBuf, AsciiBuffable},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;

pub struct Day7;

fn cost(pos: i32) -> i32 {
    pos * (pos + 1) / 2
}

impl Solution for Day7 {
    fn get_name() -> &'static str {
        "2021 Day 7: The Treachery of Whales"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Seven,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut vals: Vec<i32> = input
            .lines()
            .take(1)
            .next()
            .unwrap()
            .split(',')
            .map(|pos| ascii_buf_dec!(pos, pos.len(), 1, 2, 3, 4))
            .collect();
        vals.sort_unstable();
        let median = vals[vals.len() / 2];
        let ret = vals.iter().fold(0, |acc, v| acc + (v - median).abs());
        SolutionType::Int(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let vals: Vec<i32> = input
            .lines()
            .take(1)
            .next()
            .unwrap()
            .split(',')
            .map(|pos| ascii_buf_dec!(pos, pos.len(), 1, 2, 3, 4))
            .collect();

        let mean = vals.iter().sum::<i32>() / vals.len() as i32;

        let mut ret: i32 = i32::MAX;

        for m in [mean - 1, mean, mean + 1] {
            ret = vals
                .iter()
                .fold(0, |acc, v| acc + cost((v - m).abs()))
                .min(ret);
        }

        SolutionType::Int(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day7::get_input();
        let ret = Day7::part_1(input);
        assert_eq!(ret, SolutionType::Int(347449));
    }

    #[test]
    fn part_2() {
        let input = Day7::get_input();
        let ret = Day7::part_2(input);
        assert_eq!(ret, SolutionType::Int(98039527));
    }
}
