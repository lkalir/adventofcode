#include "aoc_2015.h"

#include <algorithm>

static const std::string day1_name("Not Quite Lisp");

auto Aoc2015Day1::name() const -> const std::string &
{
    return day1_name;
}

auto Aoc2015Day1::part1(const std::string_view &input) const -> int
{
    auto up = std::count(input.begin(), input.end(), '(');
    return 2 * up - input.length();
}

auto Aoc2015Day1::part2(const std::string_view &input) const -> int
{
    int floor = 0;

    auto basement = std::find_if(input.begin(), input.end(), [&floor](auto const &c) {
        if (c == '(')
            ++floor;
        else
            --floor;

        return floor < 0;
    });

    return std::distance(input.begin(), basement) + 1;
}
