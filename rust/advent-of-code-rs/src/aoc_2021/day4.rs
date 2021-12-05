use crate::{
    utils::asciibuf::{AsciiBuf, AsciiBuffable},
    Solution, SolutionType,
};
use advent_of_code_data_rs::get_input;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Bingo {
    items: [[(u8, bool); 5]; 5],
}

impl std::fmt::Display for Bingo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.items {
            for col in row {
                write!(f, "{:#2} ", col.0)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Bingo {
    fn new<'a>(board: impl Iterator<Item = &'a str>) -> Self {
        let mut items = [[(0, false); 5]; 5];
        for (row, line) in items.iter_mut().zip(board.skip(1).take(5)) {
            for (entry, val) in row.iter_mut().zip(line.split_whitespace()) {
                entry.0 = match val.len() {
                    1 => {
                        let buf: &AsciiBuf<1> = val.to_ascii_buf();
                        buf.as_dec_ascii::<u8>()
                    }
                    2 => {
                        let buf: &AsciiBuf<2> = val.to_ascii_buf();
                        buf.as_dec_ascii::<u8>()
                    }
                    _ => unreachable!(),
                };
            }
        }
        Self { items }
    }

    fn check_vertical(&self) -> bool {
        for idx in 0..5 {
            if self.items[0][idx].1
                && self.items[1][idx].1
                && self.items[2][idx].1
                && self.items[3][idx].1
                && self.items[4][idx].1
            {
                return true;
            }
        }

        false
    }

    fn check_horizontal(&self) -> bool {
        for idx in 0..5 {
            if self.items[idx][0].1
                && self.items[idx][1].1
                && self.items[idx][2].1
                && self.items[idx][3].1
                && self.items[idx][4].1
            {
                return true;
            }
        }

        false
    }

    fn check_diagonal(&self) -> bool {
        (self.items[0][0].1
            && self.items[1][1].1
            && self.items[2][2].1
            && self.items[3][3].1
            && self.items[4][4].1)
            || (self.items[4][0].1
                && self.items[3][1].1
                && self.items[2][2].1
                && self.items[1][3].1
                && self.items[0][4].1)
    }

    fn check_bingo(&self) -> bool {
        self.check_diagonal() || self.check_horizontal() || self.check_vertical()
    }

    fn insert(&mut self, val: u8) {
        self.items.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|item| {
                if item.0 == val {
                    item.1 = true;
                }
            })
        });
    }

    fn sum_unmarked(&self) -> u32 {
        self.items
            .iter()
            .flatten()
            .filter_map(|val| if !val.1 { Some(val.0 as u32) } else { None })
            .sum()
    }
}

pub struct Day4;

impl Solution for Day4 {
    fn get_name() -> &'static str {
        "2021 Day 4: Giant Squid"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Four,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut it = input.lines();

        let draws = it.next().unwrap().split(',').map(|draw| match draw.len() {
            1 => {
                let buf: &AsciiBuf<1> = draw.to_ascii_buf();
                buf.as_dec_ascii::<u8>()
            }
            2 => {
                let buf: &AsciiBuf<2> = draw.to_ascii_buf();
                buf.as_dec_ascii::<u8>()
            }
            _ => unreachable!(),
        });

        let mut boards: Vec<_> = it.chunks(6).into_iter().map(Bingo::new).collect();

        let mut ret = 0;

        'outer: for draw in draws {
            for board in boards.iter_mut() {
                board.insert(draw);
                if board.check_bingo() {
                    ret = board.sum_unmarked() * draw as u32;
                    break 'outer;
                }
            }
        }

        SolutionType::Uint(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut it = input.lines();

        let draws = it.next().unwrap().split(',').map(|draw| match draw.len() {
            1 => {
                let buf: &AsciiBuf<1> = draw.to_ascii_buf();
                buf.as_dec_ascii::<u8>()
            }
            2 => {
                let buf: &AsciiBuf<2> = draw.to_ascii_buf();
                buf.as_dec_ascii::<u8>()
            }
            _ => unreachable!(),
        });

        let mut boards: Vec<_> = it.chunks(6).into_iter().map(Bingo::new).collect();

        let mut ret = 0;

        for draw in draws {
            if boards.len() == 1 {
                boards[0].insert(draw);
                if boards[0].check_bingo() {
                    ret = boards[0].sum_unmarked() * draw as u32;
                    break;
                }
            } else {
                boards = boards
                    .iter_mut()
                    .filter_map(|board| {
                        board.insert(draw);
                        if !board.check_bingo() {
                            Some(*board)
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }

        SolutionType::Uint(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day4::get_input();
        let ret = Day4::part_1(input);
        assert_eq!(ret, SolutionType::Uint(21607));
    }

    #[test]
    fn part_2() {
        let input = Day4::get_input();
        let ret = Day4::part_2(input);
        assert_eq!(ret, SolutionType::Uint(19012));
    }
}
