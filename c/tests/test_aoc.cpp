#include "gtest/gtest.h"
#include <adventofcode.h>
#include <cstring>

#include "aocslns/aocdefs.h"
#include "aocslns/aocslns.h"

TEST(AdventOfCode, Aoc2015Day1Part1)
{
    const char *data = get_inputs(1, 2015);
    const char_view_t char_view = {.data = data, .len = strlen(data)};
    const aoc_day_t *fn = get_sln(2015, 1);

    ASSERT_EQ(138, fn->part1(&char_view));
}

TEST(AdventOfCode, Aoc2015Day1Part2)
{
    const char *data = get_inputs(1, 2015);
    const char_view_t char_view = {.data = data, .len = strlen(data)};
    const aoc_day_t *fn = get_sln(2015, 1);

    ASSERT_EQ(1771, fn->part2(&char_view));
}

TEST(AdventOfCode, Aoc2015Day2Part1)
{
    const char *data = get_inputs(2, 2015);
    const char_view_t char_view = {.data = data, .len = strlen(data)};
    const aoc_day_t *fn = get_sln(2015, 2);

    ASSERT_EQ(1588178, fn->part1(&char_view));
}

TEST(AdventOfCode, Aoc2015Day2Part2)
{
    const char *data = get_inputs(2, 2015);
    const char_view_t char_view = {.data = data, .len = strlen(data)};
    const aoc_day_t *fn = get_sln(2015, 2);

    ASSERT_EQ(3783758, fn->part2(&char_view));
}

auto main(int argc, char **argv) -> int
{
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
