use criterion::{criterion_group, criterion_main};

mod aoc_bench_2015 {
    use advent_of_code_rs::{aoc_2015, Solution};
    use criterion::Criterion;

    pub fn day_1_part_1(c: &mut Criterion) {
        let contents = aoc_2015::Day1::get_input();
        c.bench_function(&format!("{} part 1", aoc_2015::Day1::get_name()), |b| {
            b.iter(|| aoc_2015::Day1::part_1(contents))
        });
    }

    pub fn day_1_part_2(c: &mut Criterion) {
        let contents = aoc_2015::Day1::get_input();
        c.bench_function(&format!("{} part 2", aoc_2015::Day1::get_name()), |b| {
            b.iter(|| aoc_2015::Day1::part_2(contents))
        });
    }

    pub fn day_2_part_1(c: &mut Criterion) {
        let contents = aoc_2015::Day2::get_input();
        c.bench_function(&format!("{} part 1", aoc_2015::Day2::get_name()), |b| {
            b.iter(|| aoc_2015::Day1::part_1(contents))
        });
    }

    pub fn day_2_part_2(c: &mut Criterion) {
        let contents = aoc_2015::Day2::get_input();
        c.bench_function(&format!("{} part 2", aoc_2015::Day2::get_name()), |b| {
            b.iter(|| {
                let _ = aoc_2015::Day2::part_2(contents);
            })
        });
    }
}

mod aoc_bench_2021 {
    use advent_of_code_rs::{aoc_2021, Solution};
    use criterion::Criterion;

    pub fn day_1_part_1(c: &mut Criterion) {
        let contents = aoc_2021::Day1::get_input();
        c.bench_function(&format!("{} part 1", aoc_2021::Day1::get_name()), |b| {
            b.iter(|| aoc_2021::Day1::part_1(contents))
        });
    }

    pub fn day_1_part_2(c: &mut Criterion) {
        let contents = aoc_2021::Day1::get_input();
        c.bench_function(&format!("{} part 2", aoc_2021::Day1::get_name()), |b| {
            b.iter(|| aoc_2021::Day1::part_2(contents))
        });
    }

    pub fn day_2_part_1(c: &mut Criterion) {
        let contents = aoc_2021::Day2::get_input();
        c.bench_function(&format!("{} part 1", aoc_2021::Day2::get_name()), |b| {
            b.iter(|| aoc_2021::Day2::part_1(contents))
        });
    }

    pub fn day_2_part_2(c: &mut Criterion) {
        let contents = aoc_2021::Day2::get_input();
        c.bench_function(&format!("{} part 2", aoc_2021::Day2::get_name()), |b| {
            b.iter(|| aoc_2021::Day2::part_2(contents))
        });
    }

    pub fn day_3_part_1(c: &mut Criterion) {
        let contents = aoc_2021::Day3::get_input();
        c.bench_function(&format!("{} part 1", aoc_2021::Day3::get_name()), |b| {
            b.iter(|| aoc_2021::Day3::part_1(contents))
        });
    }

    pub fn day_3_part_2(c: &mut Criterion) {
        let contents = aoc_2021::Day3::get_input();
        c.bench_function(&format!("{} part 2", aoc_2021::Day3::get_name()), |b| {
            b.iter(|| aoc_2021::Day3::part_2(contents))
        });
    }
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
);

criterion_main!(benches_2015, benches_2021);
