use crate::Solution;
use crate::SolutionType;
use advent_of_code_data_rs::get_input;

struct Rectangle {
    l: u32,
    w: u32,
    h: u32,
}

impl Rectangle {
    fn new(length: &str, width: &str, height: &str) -> Self {
        let l = length.parse().unwrap();
        let w = width.parse().unwrap();
        let h = height.parse().unwrap();
        Self { l, w, h }
    }

    fn surface(&self) -> u32 {
        2 * (self.l * self.w + self.h * self.l + self.h * self.w)
    }

    fn smallest(&self) -> (u32, u32) {
        let mut dims = [self.l, self.w, self.h];
        dims.sort_unstable();
        (dims[0], dims[1])
    }

    fn slack(&self) -> u32 {
        let (a, b) = self.smallest();
        a * b
    }

    fn volume(&self) -> u32 {
        self.w * self.l * self.h
    }

    fn bow(&self) -> u32 {
        let (a, b) = self.smallest();
        2 * (a + b)
    }
}

pub struct Day2 {}

impl Solution for Day2 {
    fn get_name() -> &'static str {
        "2015 Day 2: I Was Told There Would Be No Math"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyFifteen,
            advent_of_code_data_rs::Day::Two,
        );
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let ret = input.lines().fold(0, |sum, l| {
            let mut dims = l.split('x');
            let l = dims.next().unwrap();
            let w = dims.next().unwrap();
            let h = dims.next().unwrap();
            let r = Rectangle::new(l, w, h);
            sum + r.surface() + r.slack()
        });

        SolutionType::Uint(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let ret = input.lines().fold(0, |sum, l| {
            let mut dims = l.split('x');
            let l = dims.next().unwrap();
            let w = dims.next().unwrap();
            let h = dims.next().unwrap();
            let r = Rectangle::new(l, w, h);
            sum + r.volume() + r.bow()
        });

        SolutionType::Uint(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2015_day2_part_1() {
        let input = Day2::get_input();
        let ret = Day2::part_1(input);
        assert_eq!(ret, SolutionType::Uint(1_588_178));
    }

    #[test]
    fn test_2015_day2_part_2() {
        let input = Day2::get_input();
        let ret = Day2::part_2(input);
        assert_eq!(ret, SolutionType::Uint(3_783_758));
    }
}
