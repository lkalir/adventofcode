use criterion::{criterion_group, criterion_main};

macro_rules! part1 {
    ($name:ident, $day:ty) => {
        pub fn $name(c: &mut Criterion) {
            let contents = <$day>::get_input();
            c.bench_function(&format!("{} part 1", <$day>::get_name()), |b| {
                b.iter(|| <$day>::part_1(contents))
            });
        }
    };
}

macro_rules! part2 {
    ($name:ident, $day:ty) => {
        pub fn $name(c: &mut Criterion) {
            let contents = <$day>::get_input();
            c.bench_function(&format!("{} part 2", <$day>::get_name()), |b| {
                b.iter(|| <$day>::part_2(contents))
            });
        }
    };
}

mod aoc_bench_2015 {
    use advent_of_code_rs::{aoc_2015, Solution};
    use criterion::Criterion;

    part1!(day_1_part_1, aoc_2015::Day1);
    part2!(day_1_part_2, aoc_2015::Day1);

    part1!(day_2_part_1, aoc_2015::Day2);
    part2!(day_2_part_2, aoc_2015::Day2);
}

mod aoc_bench_2021 {
    use advent_of_code_rs::{aoc_2021, Solution};
    use criterion::Criterion;

    part1!(day_1_part_1, aoc_2021::Day1);
    part2!(day_1_part_2, aoc_2021::Day1);

    part1!(day_2_part_1, aoc_2021::Day2);
    part2!(day_2_part_2, aoc_2021::Day2);

    part1!(day_3_part_1, aoc_2021::Day3);
    part2!(day_3_part_2, aoc_2021::Day3);

    part1!(day_4_part_1, aoc_2021::Day4);
    part2!(day_4_part_2, aoc_2021::Day4);

    part1!(day_5_part_1, aoc_2021::Day5);
    part2!(day_5_part_2, aoc_2021::Day5);

    part1!(day_6_part_1, aoc_2021::Day6);
    part2!(day_6_part_2, aoc_2021::Day6);

    part1!(day_7_part_1, aoc_2021::Day7);
    part2!(day_7_part_2, aoc_2021::Day7);

    part1!(day_8_part_1, aoc_2021::Day8);
    part2!(day_8_part_2, aoc_2021::Day8);

    part1!(day_9_part_1, aoc_2021::Day9);
    part2!(day_9_part_2, aoc_2021::Day9);
}

criterion_group!(
    benches_2015,
    aoc_bench_2015::day_1_part_1,
    aoc_bench_2015::day_1_part_2,
    aoc_bench_2015::day_2_part_1,
    aoc_bench_2015::day_2_part_2,
);

criterion_group!(
    benches_2021,
    aoc_bench_2021::day_1_part_1,
    aoc_bench_2021::day_1_part_2,
    aoc_bench_2021::day_2_part_1,
    aoc_bench_2021::day_2_part_2,
    aoc_bench_2021::day_3_part_1,
    aoc_bench_2021::day_3_part_2,
    aoc_bench_2021::day_4_part_1,
    aoc_bench_2021::day_4_part_2,
    aoc_bench_2021::day_5_part_1,
    aoc_bench_2021::day_5_part_2,
    aoc_bench_2021::day_6_part_1,
    aoc_bench_2021::day_6_part_2,
    aoc_bench_2021::day_7_part_1,
    aoc_bench_2021::day_7_part_2,
    aoc_bench_2021::day_8_part_1,
    aoc_bench_2021::day_8_part_2,
    aoc_bench_2021::day_9_part_1,
    aoc_bench_2021::day_9_part_2,
);

criterion_main!(benches_2015, benches_2021);
