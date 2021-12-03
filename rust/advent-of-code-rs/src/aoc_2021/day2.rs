use crate::Solution;
use crate::SolutionType;
use advent_of_code_data_rs::get_input;

const ASCII_D: u8 = 0x64;
const ASCII_F: u8 = 0x66;
const ASCII_U: u8 = 0x75;
const ASCII_0: u8 = 0x30;

pub struct Day2;

struct Foo<const K: usize> {
    vals: [u8; K],
}

impl<const K: usize> Foo<K> {
    pub fn first(&self) -> u8 {
        self.vals[0]
    }
    pub fn last(&self) -> u8 {
        self.vals[K - 1] - ASCII_0
    }
}

impl Solution for Day2 {
    fn get_name() -> &'static str {
        "2021 Day 2: Dive!"
    }

    fn get_input() -> &'static str {
        let x = get_input(
            advent_of_code_data_rs::Year::TwentyTwentyOne,
            advent_of_code_data_rs::Day::Two,
        );
        &x[..x.len() - 1]
    }

    fn part_1(input: &str) -> SolutionType {
        let mut horiz = 0i32;
        let mut depth = 0i32;

        for line in input.lines() {
            match unsafe {
                std::mem::transmute::<_, &Foo<1>>(&*(line.as_ptr() as *const _)).first()
            } {
                ASCII_D => {
                    let f: &Foo<6> = unsafe { &*(line.as_ptr() as *const _) };
                    depth += f.last() as i32;
                }
                ASCII_F => {
                    let f: &Foo<9> = unsafe { &*(line.as_ptr() as *const _) };
                    horiz += f.last() as i32;
                }
                ASCII_U => {
                    let f: &Foo<4> = unsafe { &*(line.as_ptr() as *const _) };
                    depth -= f.last() as i32;
                }
                _ => {}
            }
        }

        SolutionType::Int(horiz * depth)
    }

    fn part_2(input: &str) -> SolutionType {
        let mut horiz = 0i32;
        let mut depth = 0i32;
        let mut aim = 0i32;

        for line in input.lines() {
            match unsafe { &*(line.as_ptr() as *const Foo<1>) }.first() {
                ASCII_D => {
                    let f: &Foo<6> = unsafe { &*(line.as_ptr() as *const _) };
                    aim += f.last() as i32;
                }
                ASCII_F => {
                    let f: &Foo<9> = unsafe { &*(line.as_ptr() as *const _) };
                    horiz += f.last() as i32;
                    depth += aim * f.last() as i32;
                }
                ASCII_U => {
                    let f: &Foo<4> = unsafe { &*(line.as_ptr() as *const _) };
                    aim -= f.last() as i32;
                }
                _ => {}
            }
        }

        SolutionType::Int(horiz * depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2021_day1_part_1() {
        let input = Day2::get_input();
        let ret = Day2::part_1(input);
        assert_eq!(ret, SolutionType::Int(1938402));
    }

    #[test]
    fn test_2021_day1_part_2() {
        let input = Day2::get_input();
        let ret = Day2::part_2(input);
        assert_eq!(ret, SolutionType::Int(1947878632));
    }
}
