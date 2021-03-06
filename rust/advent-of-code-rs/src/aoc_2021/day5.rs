use crate::{
    macros::ascii_buf_dec,
    utils::asciibuf::{AsciiBuf, AsciiBuffable},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;

pub struct Day5;

struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(coords: impl AsRef<str>) -> Self {
        let (x, y) = coords.as_ref().split_once(",").unwrap();

        let x = ascii_buf_dec!(x, x.len(), 1, 2, 3);
        let y = ascii_buf_dec!(y, y.len(), 1, 2, 3);

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
        let mut points = [[0u16; 1000]; 1000];
        let mut cnt = 0;

        for (start, end) in input
            .lines()
            .map(|line| {
                let (first, last) = line.split_once(" -> ").unwrap();
                (Point::new(first), Point::new(last))
            })
            .filter(|(first, last)| first.is_staight(last))
        {
            for p in start.line_iter(&end) {
                points[p.y as usize][p.x as usize] += 1;

                if points[p.y as usize][p.x as usize] == 2 {
                    cnt += 1;
                }
            }
        }

        SolutionType::Uint(cnt)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut points = [[0u16; 1000]; 1000];
        let mut cnt = 0;

        for (start, end) in input.lines().map(|line| {
            let (first, last) = line.split_once(" -> ").unwrap();
            (Point::new(first), Point::new(last))
        }) {
            for p in start.line_iter(&end) {
                points[p.y as usize][p.x as usize] += 1;

                if points[p.y as usize][p.x as usize] == 2 {
                    cnt += 1;
                }
            }
        }

        SolutionType::Uint(cnt)
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
