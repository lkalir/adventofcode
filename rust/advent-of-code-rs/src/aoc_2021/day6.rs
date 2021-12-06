use crate::{
    utils::asciibuf::{AsciiBuf, AsciiBuffable},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;

pub struct Day6;

impl Solution for Day6 {
    fn get_name() -> &'static str {
        "2021 Day 6: Lanternfish"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Six,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut fishes = [0; 9];

        for num in input.split(',') {
            let l: &AsciiBuf<1> = num.to_ascii_buf();
            let val: usize = l.as_dec_ascii();
            fishes[val] += 1;
        }

        for _ in 0..80 {
            let babies = fishes[0];
            fishes.rotate_left(1);
            fishes[6] += babies;
        }

        SolutionType::Usize(fishes.iter().sum())
    }

    fn part_2(input: &str) -> SolutionType {
        let mut fishes = [0; 9];

        for num in input.split(',') {
            let l: &AsciiBuf<1> = num.to_ascii_buf();
            let val: usize = l.as_dec_ascii();
            fishes[val] += 1;
        }

        for _ in 0..256 {
            let babies = fishes[0];
            fishes.rotate_left(1);
            fishes[6] += babies;
        }

        SolutionType::Usize(fishes.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day6::get_input();
        let ret = Day6::part_1(input);
        assert_eq!(ret, SolutionType::Usize(380243));
    }

    #[test]
    fn part_2() {
        let input = Day6::get_input();
        let ret = Day6::part_2(input);
        assert_eq!(ret, SolutionType::Usize(1708791884591));
    }
}
