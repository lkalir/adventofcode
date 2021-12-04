use crate::{
    utils::{AsciiBuf, AsciiBuffable},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;
use std::convert::identity;

struct DiagnosticCalculator<const K: usize> {
    record: [u32; K],
    items: usize,
}

impl<const K: usize> DiagnosticCalculator<K> {
    pub fn new(record: [u32; K], items: usize) -> Self {
        Self { record, items }
    }

    pub fn gamma(&self) -> u16 {
        self.record
            .iter()
            .enumerate()
            .map(|(idx, val)| {
                let b = if *val > self.items as u32 / 2 { 1 } else { 0 };
                b << ((K - 1) - idx)
            })
            .sum()
    }

    pub fn epsilon(&self) -> u16 {
        let mask = (0..K).map(|i| 1 << i).sum::<u16>();
        !self.gamma() & mask
    }
}

fn gen_record<const K: usize>(input: impl Iterator<Item = impl AsRef<str>>) -> ([u32; K], usize) {
    let mut record = [0u32; K];
    let mut len = 0;

    for line in input {
        let l: &AsciiBuf<K> = line.to_ascii_buf_off(12);

        for (idx, rec) in record.iter_mut().enumerate() {
            *rec += l.as_int(idx) as u32
        }

        len += 1;
    }

    (record, len)
}

fn calc_rating(mut data: &[u16], invert: bool) -> u16 {
    let mut lbits = 0;

    for idx in (0..12).rev() {
        let mbits = lbits | (1 << idx);
        let mid = data.binary_search(&mbits).map_or_else(identity, identity);
        let ones = data.len() - mid;

        if (mid <= ones) ^ invert {
            data = &data[mid..];
            lbits = mbits;
        } else {
            data = &data[..mid];
        }

        if data.len() == 1 {
            return data[0];
        }
    }

    unreachable!()
}

pub struct Day3;

impl Solution for Day3 {
    fn get_name() -> &'static str {
        "2021 Day 3: Binary Diagnostic"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Three,
        );
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let (record, len) = gen_record(input.lines());
        let calc = DiagnosticCalculator::<12>::new(record, len);

        SolutionType::Uint(calc.gamma() as u32 * calc.epsilon() as u32)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut vals: Vec<_> = input
            .lines()
            .map(|line| u16::from_str_radix(line, 2).unwrap())
            .collect();

        vals.sort_unstable();

        let oxygen_rating = calc_rating(&vals, false) as u32;
        let co2_rating = calc_rating(&vals, true) as u32;

        SolutionType::Uint(oxygen_rating * co2_rating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day3::get_input();
        let ret = Day3::part_1(input);
        assert_eq!(ret, SolutionType::Uint(1131506));
    }

    #[test]
    fn part_2() {
        let input = Day3::get_input();
        let ret = Day3::part_2(input);
        assert_eq!(ret, SolutionType::Uint(7863147));
    }
}
