use crate::{
    utils::asciibuf::{AsciiBuf, AsciiBuffable},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;
use fnv::FnvHashMap as HashMap;

pub struct Day5;

struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(coords: impl AsRef<str>) -> Self {
        let (x, y) = coords.as_ref().split_once(",").unwrap();

        let x = match x.len() {
            1 => {
                let l: &AsciiBuf<1> = x.to_ascii_buf();
                l.as_dec_ascii()
            }
            2 => {
                let l: &AsciiBuf<2> = x.to_ascii_buf();
                l.as_dec_ascii()
            }
            3 => {
                let l: &AsciiBuf<3> = x.to_ascii_buf();
                l.as_dec_ascii()
            }
            _ => unreachable!(),
        };

        let y = match y.len() {
            1 => {
                let l: &AsciiBuf<1> = y.to_ascii_buf();
                l.as_dec_ascii()
            }
            2 => {
                let l: &AsciiBuf<2> = y.to_ascii_buf();
                l.as_dec_ascii()
            }
            3 => {
                let l: &AsciiBuf<3> = y.to_ascii_buf();
                l.as_dec_ascii()
            }
            _ => unreachable!(),
        };

        Self { x, y }
    }

    pub fn is_staight(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }

    pub fn line_iter(&self, other: &Self) -> PointIter {
        PointIter {
            x: self.x,
            y: self.y,
            x_target: other.x,
            y_target: other.y,
            done: false,
        }
    }

    pub fn as_tuple(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}

struct PointIter {
    x: u16,
    y: u16,
    x_target: u16,
    y_target: u16,
    done: bool,
}

impl Iterator for PointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            self.done = self.x == self.x_target && self.y == self.y_target;

            let p = Point {
                x: self.x,
                y: self.y,
            };

            match self.x.cmp(&self.x_target) {
                std::cmp::Ordering::Less => self.x += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => self.x -= 1,
            }

            match self.y.cmp(&self.y_target) {
                std::cmp::Ordering::Less => self.y += 1,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => self.y -= 1,
            }

            Some(p)
        }
    }
}

impl Solution for Day5 {
    fn get_name() -> &'static str {
        "2021 Day 5: Hydrothermal Venture"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Five,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut points: HashMap<(u16, u16), u16> = HashMap::default();
        for (start, end) in input
            .lines()
            .map(|line| {
                let (first, last) = line.split_once(" -> ").unwrap();
                (Point::new(first), Point::new(last))
            })
            .filter(|(first, last)| first.is_staight(last))
        {
            for p in start.line_iter(&end) {
                *points.entry(p.as_tuple()).or_insert(0) += 1;
            }
        }

        let ret = points.iter().filter(|(_, &val)| val >= 2).count();
        SolutionType::Uint(ret as _)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut points: HashMap<(u16, u16), u16> = HashMap::default();
        for (start, end) in input.lines().map(|line| {
            let (first, last) = line.split_once(" -> ").unwrap();
            (Point::new(first), Point::new(last))
        }) {
            for p in start.line_iter(&end) {
                *points.entry(p.as_tuple()).or_insert(0) += 1;
            }
        }

        let ret = points.iter().filter(|(_, &val)| val >= 2).count();
        SolutionType::Uint(ret as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day5::get_input();
        let ret = Day5::part_1(input);
        assert_eq!(ret, SolutionType::Uint(4421));
    }

    #[test]
    fn part_2() {
        let input = Day5::get_input();
        let ret = Day5::part_2(input);
        assert_eq!(ret, SolutionType::Uint(18674));
    }
}
