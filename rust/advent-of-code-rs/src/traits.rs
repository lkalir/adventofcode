#[derive(Eq, PartialEq, Debug)]
pub enum SolutionType {
    Int(i32),
    Uint(u32),
}

pub trait Solution {
    fn get_name() -> &'static str;
    fn get_input() -> &'static str;
    fn part_1(input: &str) -> SolutionType;
    fn part_2(input: &str) -> SolutionType;
}


