#include "gtest/gtest.h"
#include <adventofcode.h>
#include <cstring>

#include "../src/aoc_2015/aoc_2015.h"

#include <aoc.h>

TEST(AdventOfCode, Aoc2015Day1Part1)
{
    const char *data = get_inputs(1, 2015);
    const char_view_t char_view = {.data = data, .len = strlen(data)};
    ASSERT_EQ(138, aoc_2015_day_1.part1(&char_view));
}

TEST(AdventOfCode, Aoc2015Day1Part2)
{
    const char *data = get_inputs(1, 2015);
    const char_view_t char_view = {.data = data, .len = strlen(data)};
    ASSERT_EQ(1771, aoc_2015_day_1.part2(&char_view));
}

auto main(int argc, char **argv) -> int
{
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
