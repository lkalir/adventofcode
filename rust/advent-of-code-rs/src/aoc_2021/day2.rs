use crate::{
    utils::asciibuf::{AsciiBuf, AsciiBuffable, ASCII_D, ASCII_F, ASCII_U},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;
use std::hint::unreachable_unchecked;

pub struct Day2;

impl Solution for Day2 {
    fn get_name() -> &'static str {
        "2021 Day 2: Dive!"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Two,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut horiz = 0i32;
        let mut depth = 0i32;

        for line in input.lines() {
            let l: &AsciiBuf<1> = line.to_ascii_buf();

            match l[0] {
                ASCII_D => {
                    let f: &AsciiBuf<6> = line.to_ascii_buf();
                    depth += f.as_int(f.len() - 1) as i32;
                }
                ASCII_F => {
                    let f: &AsciiBuf<9> = line.to_ascii_buf();
                    horiz += f.as_int(f.len() - 1) as i32;
                }
                ASCII_U => {
                    let f: &AsciiBuf<4> = line.to_ascii_buf();
                    depth -= f.as_int(f.len() - 1) as i32;
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        SolutionType::Int(horiz * depth)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut horiz = 0i32;
        let mut depth = 0i32;
        let mut aim = 0i32;

        for line in input.lines() {
            let l: &AsciiBuf<1> = line.to_ascii_buf();

            match l[0] {
                ASCII_D => {
                    let f: &AsciiBuf<6> = line.to_ascii_buf();
                    aim += f.as_int(f.len() - 1) as i32;
                }
                ASCII_F => {
                    let f: &AsciiBuf<9> = line.to_ascii_buf();
                    horiz += f.as_int(f.len() - 1) as i32;
                    depth += aim * f.as_int(f.len() - 1) as i32;
                }
                ASCII_U => {
                    let f: &AsciiBuf<4> = line.to_ascii_buf();
                    aim -= f.as_int(f.len() - 1) as i32;
                }
                _ => unsafe { unreachable_unchecked() },
            }
        }

        SolutionType::Int(horiz * depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day2::get_input();
        let ret = Day2::part_1(input);
        assert_eq!(ret, SolutionType::Int(1938402));
    }

    #[test]
    fn part_2() {
        let input = Day2::get_input();
        let ret = Day2::part_2(input);
        assert_eq!(ret, SolutionType::Int(1947878632));
    }
}
