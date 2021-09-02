#include <adventofcode.h>
#include <benchmark/benchmark.h>
#include <cstring>

#include "../src/aoc_2015/aoc_2015.h"
#include <aoc.h>

static void BM_Aoc2015Day1Part1(benchmark::State &state)
{
    const char *data = get_inputs(1, 2015);
    const char_view_t data_view = {.data = data, .len = strlen(data)};
    for (auto _ : state)
        benchmark::DoNotOptimize(aoc_2015_day_1_part_1(&data_view));
}

static void BM_Aoc2015Day1Part2(benchmark::State &state)
{
    const char *data = get_inputs(1, 2015);
    const char_view_t data_view = {.data = data, .len = strlen(data)};
    for (auto _ : state)
        benchmark::DoNotOptimize(aoc_2015_day_1_part_2(&data_view));
}

BENCHMARK(BM_Aoc2015Day1Part1);
BENCHMARK(BM_Aoc2015Day1Part2);

BENCHMARK_MAIN();
