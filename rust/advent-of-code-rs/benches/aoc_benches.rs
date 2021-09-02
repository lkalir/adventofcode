use advent_of_code_rs::{aoc_2015, Solution};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn aoc_2015_day_1_part_1(c: &mut Criterion) {
    let contents = aoc_2015::Day1::get_input();
    c.bench_function(&format!("{} part 1", aoc_2015::Day1::get_name()), |b| {
        b.iter(|| aoc_2015::Day1::part_1(contents))
    });
}

fn aoc_2015_day_1_part_2(c: &mut Criterion) {
    let contents = aoc_2015::Day1::get_input();
    c.bench_function(&format!("{} part 2", aoc_2015::Day1::get_name()), |b| {
        b.iter(|| aoc_2015::Day1::part_2(contents))
    });
}

fn aoc_2015_day_2_part_1(c: &mut Criterion) {
    let contents = aoc_2015::Day2::get_input();
    c.bench_function(&format!("{} part 1", aoc_2015::Day2::get_name()), |b| {
        b.iter(|| aoc_2015::Day1::part_1(contents))
    });
}

fn aoc_2015_day_2_part_2(c: &mut Criterion) {
    let contents = aoc_2015::Day2::get_input();
    c.bench_function(&format!("{} part 2", aoc_2015::Day2::get_name()), |b| {
        b.iter(|| {
            let _ = aoc_2015::Day2::part_2(contents);
        })
    });
}

criterion_group!(
    benches,
    aoc_2015_day_1_part_1,
    aoc_2015_day_1_part_2,
    aoc_2015_day_2_part_1,
    aoc_2015_day_2_part_2
);
criterion_main!(benches);
