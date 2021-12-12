use crate::{Solution, SolutionType};
use advent_of_code_data_rs::get_input;

pub struct Day10;

#[derive(Debug)]
struct Stack<T: Default + Copy, const K: usize> {
    stack: [T; K],
    sp: usize,
}

impl<T: Default + Copy, const K: usize> Stack<T, K> {
    fn new() -> Self {
        Self {
            stack: [T::default(); K],
            sp: 0,
        }
    }

    fn push(&mut self, val: T) {
        self.stack[self.sp] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> T {
        self.sp -= 1;
        self.stack[self.sp]
    }

    fn peek(&self, idx: usize) -> T {
        self.stack[self.sp - idx - 1]
    }
}

const SCORE_PAREN: usize = 3;
const SCORE_BRACKET: usize = 57;
const SCORE_BRACE: usize = 1197;
const SCORE_ANGLE_BRACKET: usize = 25137;

const SCORE_COMPLETE_PAREN: usize = 1;
const SCORE_COMPLETE_BRACKET: usize = 2;
const SCORE_COMPLETE_BRACE: usize = 3;
const SCORE_COMPLETE_ANGLE_BRACKET: usize = 4;

impl Solution for Day10 {
    fn get_name() -> &'static str {
        "2021 Day 10: Syntax Scoring"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Ten,
        )
        .unwrap();
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let ret = input
            .lines()
            .map(|line| {
                let mut stack = Stack::<char, 128>::new();
                let mut ret = 0;

                for c in line.chars() {
                    match c {
                        '(' | '[' | '{' | '<' => stack.push(c),
                        ')' | '}' | ']' | '>' => match (stack.pop(), c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {}
                            (_, ')') => {
                                ret = SCORE_PAREN;
                                break;
                            }
                            (_, '}') => {
                                ret = SCORE_BRACE;
                                break;
                            }
                            (_, ']') => {
                                ret = SCORE_BRACKET;
                                break;
                            }
                            (_, '>') => {
                                ret = SCORE_ANGLE_BRACKET;
                                break;
                            }
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }

                ret
            })
            .sum();
        SolutionType::Usize(ret)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut ret: Vec<_> = input
            .lines()
            .filter_map(|line| {
                let mut stack = Stack::<char, 128>::new();

                for c in line.chars() {
                    match c {
                        '(' | '[' | '{' | '<' => stack.push(c),
                        ')' | '}' | ']' | '>' => match (stack.pop(), c) {
                            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {}
                            (_, ')' | '}' | ']' | '>') => return None,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                }

                Some(stack)
            })
            .map(|stack| {
                (0..stack.sp).fold(0, |acc, i| {
                    acc * 5
                        + match stack.peek(i) {
                            '(' => SCORE_COMPLETE_PAREN,
                            '[' => SCORE_COMPLETE_BRACKET,
                            '{' => SCORE_COMPLETE_BRACE,
                            '<' => SCORE_COMPLETE_ANGLE_BRACKET,
                            _ => unreachable!(),
                        }
                })
            })
            .collect();
        ret.sort_unstable();

        SolutionType::Usize(ret[ret.len() / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = Day10::get_input();
        let ret = Day10::part_1(input);
        assert_eq!(ret, SolutionType::Usize(388713));
    }

    #[test]
    fn part_2() {
        let input = Day10::get_input();
        let ret = Day10::part_2(input);
        assert_eq!(ret, SolutionType::Usize(3539961434));
    }
}
