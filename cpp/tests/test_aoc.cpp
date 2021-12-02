#include "gtest/gtest.h"
#include <aoc.h>

TEST(AdventOfCode, Aoc2015Day1Part1)
{
    auto dut = AocSolution<1, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(138, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day1Part2)
{
    auto dut = AocSolution<1, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(1771, dut.part2(data));
}

TEST(AdventOfCode, Aoc2015Day2Part1)
{
    auto dut = AocSolution<2, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(1588178, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day2Part2)
{
    auto dut = AocSolution<2, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(3783758, dut.part2(data));
}

TEST(AdventOfCode, Aoc2015Day3Part1)
{
    auto dut = AocSolution<3, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(2565, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day3Part2)
{
    auto dut = AocSolution<3, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(2639, dut.part2(data));
}

TEST(AdventOfCode, Aoc2015Day4Part1)
{
    auto dut = AocSolution<4, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(282749, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day4Part2)
{
    auto dut = AocSolution<4, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(9962624, dut.part2(data));
}

TEST(AdventOfCode, Aoc2015Day5Part1)
{
    auto dut = AocSolution<5, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(255, dut.part1(data));
}

TEST(AdventOfCode, Aoc2015Day5Part2)
{
    auto dut = AocSolution<5, 2015>();
    auto data = dut.inputs();
    ASSERT_EQ(55, dut.part2(data));
}

TEST(AdventOfCode, Aoc2019Day1Part1)
{
    auto dut = AocSolution<1, 2019>();
    auto data = dut.inputs();
    ASSERT_EQ(3318632, dut.part1(data));
}

TEST(AdventOfCode, Aoc2019Day1Part2)
{
    auto dut = AocSolution<1, 2019>();
    auto data = dut.inputs();
    ASSERT_EQ(4975084, dut.part2(data));
}

auto main(int argc, char **argv) -> int
{
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
