use crate::{utils::asciibuf::ASCII_0, Solution, SolutionType};
use advent_of_code_data_rs::get_input;

pub struct Day9;

#[derive(Default)]
struct NeighborIter {
    neighbors: [(usize, usize); 4],
    len: usize,
    idx: usize,
}

#[derive(Debug)]
struct MaxBuffer<T: Default + Copy + std::cmp::Ord, const K: usize> {
    buf: [T; K],
}

impl<T: Default + Copy + std::cmp::Ord, const K: usize> MaxBuffer<T, K> {
    fn new() -> Self {
        Self {
            buf: [T::default(); K],
        }
    }

    fn insert(&mut self, val: T) {
        let min = self.buf.iter_mut().min().unwrap();

        if *min < val {
            *min = val;
        }
    }

    fn iter(&self) -> std::slice::Iter<'_, T> {
        self.buf.iter()
    }
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.len {
            None
        } else {
            let ret = self.neighbors[self.idx];
            self.idx += 1;
            Some(ret)
        }
    }
}

trait Neighborable {
    fn neighbors(&self, row: usize, col: usize) -> NeighborIter;
}

impl Neighborable for [[u8; 100]; 100] {
    fn neighbors(&self, row: usize, col: usize) -> NeighborIter {
        let mut ret = NeighborIter::default();
        match (row, col) {
            (0, 0) => {
                ret.neighbors[0] = (0, 1);
                ret.neighbors[1] = (1, 0);
                ret.len = 2;
            }
            (99, 99) => {
                ret.neighbors[0] = (99, 98);
                ret.neighbors[1] = (98, 99);
                ret.len = 2;
            }
            (99, 0) => {
                ret.neighbors[0] = (99, 1);
                ret.neighbors[1] = (98, 1);
                ret.len = 2;
            }
            (0, 99) => {
                ret.neighbors[0] = (0, 98);
                ret.neighbors[1] = (1, 99);
                ret.len = 2;
            }
            (0, col) => {
                ret.neighbors[0] = (1, col);
                ret.neighbors[1] = (0, col - 1);
                ret.neighbors[2] = (0, col + 1);
                ret.len = 3;
            }
            (99, col) => {
                ret.neighbors[0] = (98, col);
                ret.neighbors[1] = (99, col - 1);
                ret.neighbors[2] = (99, col + 1);
                ret.len = 3;
            }
            (row, 0) => {
                ret.neighbors[0] = (row - 1, 0);
                ret.neighbors[1] = (row + 1, 0);
                ret.neighbors[2] = (row, 1);
                ret.len = 3;
            }
            (row, 99) => {
                ret.neighbors[0] = (row - 1, 99);
                ret.neighbors[1] = (row + 1, 99);
                ret.neighbors[2] = (row, 98);
                ret.len = 3;
            }
            (row, col) => {
                ret.neighbors[0] = (row - 1, col);
                ret.neighbors[1] = (row + 1, col);
                ret.neighbors[2] = (row, col - 1);
                ret.neighbors[3] = (row, col + 1);
                ret.len = 4;
            }
        };

        ret
    }
}

fn is_low_point(data: &[[u8; 100]; 100], row: usize, col: usize) -> bool {
    let point = data[row][col];
    data.neighbors(row, col)
        .filter(|&(x, y)| point >= data[x][y])
        .count()
        == 0
}

fn basin_size(
    data: &[[u8; 100]; 100],
    visited: &mut [[bool; 100]; 100],
    row: usize,
    col: usize,
) -> usize {
    if data[row][col] == 9 || visited[row][col] {
        0
    } else {
        visited[row][col] = true;
        data.neighbors(row, col)
            .filter(|&(r, c)| data[r][c] != 9)
            .fold(1, |size, (r, c)| size + basin_size(data, visited, r, c))
    }
}

impl Solution for Day9 {
    fn get_name() -> &'static str {
        "2021 Day 9: Smoke Basin"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Nine,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut data = [[0u8; 100]; 100];

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                data[row][col] = c as u8 - ASCII_0;
            }
        }

        let mut ret = 0;

        for row in 0..100 {
            for col in 0..100 {
                if is_low_point(&data, row, col) {
                    ret += 1 + data[row][col] as u32;
                }
            }
        }

        SolutionType::Uint(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut data = [[0u8; 100]; 100];

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                data[row][col] = c as u8 - ASCII_0;
            }
        }

        let mut buf = MaxBuffer::<usize, 3>::new();
        let mut traversed = [[false; 100]; 100];

        for row in 0..100 {
            for col in 0..100 {
                if is_low_point(&data, row, col) {
                    let basin = basin_size(&data, &mut traversed, row, col);
                    buf.insert(basin);
                }
            }
        }

        SolutionType::Usize(buf.iter().product())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day9::get_input();
        let ret = Day9::part_1(input);
        assert_eq!(ret, SolutionType::Uint(504));
    }

    #[test]
    fn part_2() {
        let input = Day9::get_input();
        let ret = Day9::part_2(input);
        assert_eq!(ret, SolutionType::Usize(1558722));
    }
}
