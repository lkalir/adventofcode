use advent_of_code_data_rs::Day;

#[derive(Eq, PartialEq, Debug)]
pub enum SolutionType {
    Int(i32),
    Uint(u32),
    Usize(usize),
}

pub trait Solution {
    fn get_name() -> &'static str;
    fn get_input() -> &'static str;
    fn part_1(input: &str) -> SolutionType;
    fn part_2(input: &str) -> SolutionType;
}

pub trait SolutionYear {
    fn get_sln(day: Day) -> Option<(SolutionType, SolutionType)>;
}
