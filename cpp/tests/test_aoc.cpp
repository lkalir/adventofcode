#include "gtest/gtest.h"
#include <adventofcode.h>

#include "../src/aoc_2015/aoc_2015.h"

TEST(AdventOfCode, Aoc2015Day1Part1)
{
    auto dut = Aoc2015Day1();
    auto data = dut.inputs();
    ASSERT_EQ(138, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day1Part2)
{
    auto dut = Aoc2015Day1();
    auto data = dut.inputs();
    ASSERT_EQ(1771, dut.part2(data));
}

TEST(AdventOfCode, Aoc2015Day2Part1)
{
    auto dut = Aoc2015Day2();
    auto data = dut.inputs();
    ASSERT_EQ(1588178, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day2Part2)
{
    auto dut = Aoc2015Day2();
    auto data = dut.inputs();
    ASSERT_EQ(3783758, dut.part2(data));
}

auto main(int argc, char **argv) -> int
{
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
