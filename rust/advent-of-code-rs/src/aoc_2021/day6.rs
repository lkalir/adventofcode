use crate::{utils::asciibuf::ASCII_0, Solution, SolutionType};
use advent_of_code_data_rs::get_input;
use std::intrinsics::copy;

pub struct Day6;

// For some reason this results in faster codegen than const generics
fn simulate_fish(input: &str, cycles: usize) -> SolutionType {
    let mut fishes = [0; 9];

    for (_, num) in input.chars().enumerate().filter(|(idx, _)| idx % 2 == 0) {
        let val = num as usize - ASCII_0 as usize;
        fishes[val] += 1;
    }

    for _ in 0..cycles {
        let babies = fishes[0];
        unsafe { copy(fishes[1..9].as_ptr(), fishes[0..8].as_mut_ptr(), 8) };
        fishes[8] = babies;
        fishes[6] += babies;
    }

    SolutionType::Usize(fishes.iter().sum())
}

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
        simulate_fish(input, 80)
    }

    fn part_2(input: &str) -> SolutionType {
        simulate_fish(input, 256)
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
