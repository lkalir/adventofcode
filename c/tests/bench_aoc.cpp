#include <adventofcode.h>
#include <benchmark/benchmark.h>
#include <cstring>

#include "../src/aoc_2015/aoc_2015.h"
#include <aoc.h>

#define BM_AOCSLN(daynum, year)                                                    \
    static auto BM_Aoc##year##Day##daynum##Part1(benchmark::State &state)->void    \
    {                                                                              \
        const char *data = get_inputs(daynum, year);                               \
        const char_view_t data_view = {.data = data, .len = strlen(data)};         \
        for (auto _ : state)                                                       \
            benchmark::DoNotOptimize(aoc_##year##_day_##daynum.part1(&data_view)); \
    }                                                                              \
    static auto BM_Aoc##year##Day##daynum##Part2(benchmark::State &state)->void    \
    {                                                                              \
        const char *data = get_inputs(daynum, year);                               \
        const char_view_t data_view = {.data = data, .len = strlen(data)};         \
        for (auto _ : state)                                                       \
            benchmark::DoNotOptimize(aoc_##year##_day_##daynum.part2(&data_view)); \
    }                                                                              \
    BENCHMARK(BM_Aoc##year##Day##daynum##Part1);                                   \
    BENCHMARK(BM_Aoc##year##Day##daynum##Part2);

BM_AOCSLN(1, 2015);
BM_AOCSLN(2, 2015);

BENCHMARK_MAIN();
