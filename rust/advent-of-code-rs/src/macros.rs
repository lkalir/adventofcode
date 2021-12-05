macro_rules! add_sln {
    ($day:ident, $input:expr) => {
        Some(($day::part_1($input), $day::part_2($input)))
    };
}

pub(crate) use add_sln;
