use crate::Solution;
use crate::SolutionType;
use advent_of_code_data_rs::get_input;

pub struct Day2;

impl Solution for Day2 {
    fn get_name() -> &'static str {
        "2021 Day 2: Dive!"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Two,
        );
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut horiz = 0i32;
        let mut depth = 0i32;

        for line in input.lines() {
            match line.chars().next() {
                Some('f') => {
                    let val = line.chars().last().unwrap().to_digit(10).unwrap();
                    horiz += val as i32;
                }
                Some('u') => {
                    let val = line.chars().last().unwrap().to_digit(10).unwrap();
                    depth -= val as i32;
                }
                Some('d') => {
                    let val = line.chars().last().unwrap().to_digit(10).unwrap();
                    depth += val as i32;
                }
                _ => {}
            }
        }

        SolutionType::Int(horiz * depth)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut horiz = 0i32;
        let mut depth = 0i32;
        let mut aim = 0i32;

        for line in input.lines() {
            match line.chars().next() {
                Some('f') => {
                    let val = line.chars().last().unwrap().to_digit(10).unwrap();
                    horiz += val as i32;
                    depth += aim * val as i32;
                }
                Some('u') => {
                    let val = line.chars().last().unwrap().to_digit(10).unwrap();
                    aim -= val as i32;
                }
                Some('d') => {
                    let val = line.chars().last().unwrap().to_digit(10).unwrap();
                    aim += val as i32;
                }
                _ => {}
            }
        }

        SolutionType::Int(horiz * depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_day1_part_1() {
        let input = Day2::get_input();
        let ret = Day2::part_1(input);
        assert_eq!(ret, SolutionType::Int(1938402));
    }

    #[test]
    fn test_2021_day1_part_2() {
        let input = Day2::get_input();
        let ret = Day2::part_2(input);
        assert_eq!(ret, SolutionType::Int(1947878632));
    }
}
