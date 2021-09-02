#include <adventofcode.h>
#include <benchmark/benchmark.h>

#include "../src/aoc_2015/aoc_2015.h"

static void BM_Aoc2015Day1Part1(benchmark::State &state)
{
    auto dut = Aoc2015Day1();
    auto data = dut.inputs();

    for (auto _ : state)
        benchmark::DoNotOptimize(dut.part1(data));
}

static void BM_Aoc2015Day1Part2(benchmark::State &state)
{
    auto dut = Aoc2015Day1();
    auto data = dut.inputs();

    for (auto _ : state)
        benchmark::DoNotOptimize(dut.part2(data));
}

static void BM_Aoc2015Day2Part1(benchmark::State &state)
{
    auto dut = Aoc2015Day2();
    auto data = dut.inputs();

    for (auto _ : state)
        benchmark::DoNotOptimize(dut.part1(data));
}

static void BM_Aoc2015Day2Part2(benchmark::State &state)
{
    auto dut = Aoc2015Day2();
    auto data = dut.inputs();

    for (auto _ : state)
        benchmark::DoNotOptimize(dut.part2(data));
}

BENCHMARK(BM_Aoc2015Day1Part1);
BENCHMARK(BM_Aoc2015Day1Part2);
BENCHMARK(BM_Aoc2015Day2Part1);
BENCHMARK(BM_Aoc2015Day2Part2);

BENCHMARK_MAIN();
