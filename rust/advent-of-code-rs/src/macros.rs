macro_rules! add_sln {
    ($day:ident, $input:expr) => {
        Some(($day::part_1($input), $day::part_2($input)))
    };
}

pub(crate) use add_sln;

macro_rules! ascii_buf_dec {
    ($input:ident, $len:expr, $($val:expr),+) => {
        match $len {
        $(
            $val => {
                let l: &AsciiBuf<$val> = $input.to_ascii_buf();
                l.as_dec_ascii()
            }
        )+
            _ => unsafe { std::hint::unreachable_unchecked() }

        }
    };
}

pub(crate) use ascii_buf_dec;
