#include <aoc.h>
#include <benchmark/benchmark.h>

#define BM_AOCSLN(day, year)                                                 \
    static auto BM_Aoc##year##Day##day##Part1(benchmark::State &state)->void \
    {                                                                        \
        AocSolution<day, year> dut;                                          \
        auto data = dut.inputs();                                            \
        for (auto _ : state)                                                 \
            benchmark::DoNotOptimize(dut.part1(data));                       \
    }                                                                        \
    static auto BM_Aoc##year##Day##day##Part2(benchmark::State &state)->void \
    {                                                                        \
        AocSolution<day, year> dut;                                          \
        auto data = dut.inputs();                                            \
        for (auto _ : state)                                                 \
            benchmark::DoNotOptimize(dut.part2(data));                       \
    }                                                                        \
    BENCHMARK(BM_Aoc##year##Day##day##Part1);                                \
    BENCHMARK(BM_Aoc##year##Day##day##Part2);

BM_AOCSLN(1, 2015)
BM_AOCSLN(2, 2015)
BM_AOCSLN(3, 2015)
BM_AOCSLN(4, 2015)
BM_AOCSLN(5, 2015)

BENCHMARK_MAIN();
