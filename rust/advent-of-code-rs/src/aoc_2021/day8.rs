use crate::{utils::asciibuf::ASCII_a, Solution, SolutionType};
use advent_of_code_data_rs::get_input;

pub struct Day8;

trait IntSet {
    fn activate(input: &str) -> Self;
    fn set(&mut self, c: char);
    fn is_set(&self, c: char) -> bool;
    fn intersect(&self, other: u64) -> u64;
    fn len(&self) -> usize;
}

impl IntSet for u64 {
    fn activate(input: &str) -> Self {
        let mut ret = 0;

        for c in input.chars() {
            ret.set(c);
        }

        ret
    }

    fn is_set(&self, c: char) -> bool {
        let off = c as u8 - ASCII_a;
        (self & (0xFF << (off * 8))) > 0
    }

    fn set(&mut self, c: char) {
        let off = c as u8 - ASCII_a;
        *self |= 0xFF << (off * 8);
    }

    fn intersect(&self, other: u64) -> u64 {
        let mut ret = 0;

        for c in 'a'..='g' {
            if let (true, true) = (self.is_set(c), other.is_set(c)) {
                ret.set(c)
            }
        }

        ret
    }

    fn len(&self) -> usize {
        ('a'..='g').filter(|&c| self.is_set(c)).count()
    }
}

fn make_set(input: &str) -> [u64; 7] {
    let mut sets = [0u64; 7];
    for number in input.split(' ') {
        for c in number.chars() {
            sets[number.len() - 1].set(c);
        }
    }
    sets
}

impl Solution for Day8 {
    fn get_name() -> &'static str {
        "2021 Day 8: Seven Segment Search"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Eight,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let ret = input
            .lines()
            .map(|line| line.split_once('|').unwrap().1)
            .map(|output| {
                output
                    .split(' ')
                    .filter(|display| matches!(display.len(), 2..=4 | 7))
                    .count()
            })
            .sum();
        SolutionType::Usize(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let ret = input
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|(input, output)| {
                let inset = make_set(input);
                let mut outset = [0; 4];

                for (storage, digit) in outset.iter_mut().zip(output.split(' ').skip(1)) {
                    *storage = IntSet::activate(digit);
                }

                outset.iter().fold(0, |acc, digit| {
                    let d = match (
                        digit.len(),
                        digit.intersect(inset[3]).len(),
                        digit.intersect(inset[1]).len(),
                    ) {
                        (2, _, _) => 1,
                        (3, _, _) => 7,
                        (4, _, _) => 4,
                        (7, _, _) => 8,
                        (5, 2, _) => 2,
                        (5, 3, 1) => 5,
                        (5, 3, 2) => 3,
                        (6, 4, _) => 9,
                        (6, 3, 1) => 6,
                        (6, 3, 2) => 0,
                        _ => unreachable!(),
                    };
                    acc * 10 + d
                })
            })
            .sum();

        SolutionType::Uint(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day8::get_input();
        let ret = Day8::part_1(input);
        assert_eq!(ret, SolutionType::Usize(470));
    }

    #[test]
    fn part_2() {
        let input = Day8::get_input();
        let ret = Day8::part_2(input);
        assert_eq!(ret, SolutionType::Uint(989396));
    }
}
