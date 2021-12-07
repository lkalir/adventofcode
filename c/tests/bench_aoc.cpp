#include <adventofcode.h>
#include <benchmark/benchmark.h>
#include <cstring>

#include "aocslns/aocdefs.h"
#include "aocslns/aocslns.h"

#define BM_AOCSLN(daynum, year)                                                 \
    static auto BM_Aoc##year##Day##daynum##Part1(benchmark::State &state)->void \
    {                                                                           \
        const char *data = get_inputs(daynum, year);                            \
        const char_view_t data_view = {.data = data, .len = strlen(data)};      \
        const aoc_day_t *fn = get_sln(year, daynum);                            \
        for (auto _ : state)                                                    \
            benchmark::DoNotOptimize(fn->part1(&data_view));                    \
    }                                                                           \
    static auto BM_Aoc##year##Day##daynum##Part2(benchmark::State &state)->void \
    {                                                                           \
        const char *data = get_inputs(daynum, year);                            \
        const char_view_t data_view = {.data = data, .len = strlen(data)};      \
        const aoc_day_t *fn = get_sln(year, daynum);                            \
        for (auto _ : state)                                                    \
            benchmark::DoNotOptimize(fn->part2(&data_view));                    \
    }                                                                           \
    BENCHMARK(BM_Aoc##year##Day##daynum##Part1);                                \
    BENCHMARK(BM_Aoc##year##Day##daynum##Part2);

BM_AOCSLN(1, 2015);
BM_AOCSLN(2, 2015);

BENCHMARK_MAIN();
