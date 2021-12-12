use crate::{utils::asciibuf::ASCII_0, Solution, SolutionType};
use advent_of_code_data_rs::get_input;

pub struct Day11;

fn increment(
    data: &mut [[u8; 10]; 10],
    flashed: &mut [[bool; 10]; 10],
    row: usize,
    col: usize,
) -> usize {
    if flashed[row][col] {
        0
    } else if data[row][col] != 9 {
        data[row][col] += 1;
        0
    } else {
        data[row][col] = 0;
        flashed[row][col] = true;

        1 + match (row, col) {
            (0, 0) => {
                increment(data, flashed, 1, 0)
                    + increment(data, flashed, 0, 1)
                    + increment(data, flashed, 1, 1)
            }
            (9, 9) => {
                increment(data, flashed, 8, 9)
                    + increment(data, flashed, 9, 8)
                    + increment(data, flashed, 8, 8)
            }
            (9, 0) => {
                increment(data, flashed, 9, 1)
                    + increment(data, flashed, 8, 0)
                    + increment(data, flashed, 8, 1)
            }
            (0, 9) => {
                increment(data, flashed, 1, 9)
                    + increment(data, flashed, 0, 8)
                    + increment(data, flashed, 1, 8)
            }
            (0, c) => {
                increment(data, flashed, 0, c - 1)
                    + increment(data, flashed, 0, c + 1)
                    + increment(data, flashed, 1, c - 1)
                    + increment(data, flashed, 1, c)
                    + increment(data, flashed, 1, c + 1)
            }
            (9, c) => {
                increment(data, flashed, 9, c - 1)
                    + increment(data, flashed, 9, c + 1)
                    + increment(data, flashed, 8, c - 1)
                    + increment(data, flashed, 8, c)
                    + increment(data, flashed, 8, c + 1)
            }
            (r, 0) => {
                increment(data, flashed, r - 1, 0)
                    + increment(data, flashed, r + 1, 0)
                    + increment(data, flashed, r - 1, 1)
                    + increment(data, flashed, r, 1)
                    + increment(data, flashed, r + 1, 1)
            }
            (r, 9) => {
                increment(data, flashed, r - 1, 9)
                    + increment(data, flashed, r + 1, 9)
                    + increment(data, flashed, r - 1, 8)
                    + increment(data, flashed, r, 8)
                    + increment(data, flashed, r + 1, 8)
            }
            (r, c) => {
                increment(data, flashed, r - 1, c - 1)
                    + increment(data, flashed, r - 1, c)
                    + increment(data, flashed, r - 1, c + 1)
                    + increment(data, flashed, r, c - 1)
                    + increment(data, flashed, r, c + 1)
                    + increment(data, flashed, r + 1, c - 1)
                    + increment(data, flashed, r + 1, c)
                    + increment(data, flashed, r + 1, c + 1)
            }
        }
    }
}

impl Solution for Day11 {
    fn get_name() -> &'static str {
        "2021 Day 11: Dumbo Octopus"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Eleven,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut flashes = 0;
        let mut octopi = [[0u8; 10]; 10];

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                octopi[row][col] = c as u8 - ASCII_0;
            }
        }

        for _ in 0..100 {
            let mut flashed = [[false; 10]; 10];
            for row in 0..10 {
                for col in 0..10 {
                    flashes += increment(&mut octopi, &mut flashed, row, col);
                }
            }
        }

        SolutionType::Usize(flashes)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut octopi = [[0u8; 10]; 10];

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                octopi[row][col] = c as u8 - ASCII_0;
            }
        }

        let mut ret = 0;

        for i in 0.. {
            let mut flashes = 0;
            let mut flashed = [[false; 10]; 10];

            for row in 0..10 {
                for col in 0..10 {
                    flashes += increment(&mut octopi, &mut flashed, row, col);
                }
            }

            if flashes == 100 {
                ret = i + 1;
                break;
            }
        }

        SolutionType::Usize(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day11::get_input();
        let ret = Day11::part_1(input);
        assert_eq!(ret, SolutionType::Usize(1613));
    }

    #[test]
    fn part_2() {
        let input = Day11::get_input();
        let ret = Day11::part_2(input);
        assert_eq!(ret, SolutionType::Usize(510));
    }
}
