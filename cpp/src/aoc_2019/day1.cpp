#include <algorithm>
#include <aoc.h>
#include <cstdio>
#include <numeric>
#include <string>
#include <string_view>
#include <utility>

#include "util.h"

static const std::string day1_name("The Tyranny of the Rocket Equation");

template <> auto AocSolution<1, 2019>::name() const -> const std::string &
{
    return day1_name;
}

static auto fuel(const int mass) -> int
{
    return (mass / 3) - 2;
}

template <> auto AocSolution<1, 2019>::part1(const std::string_view &input) const -> int
{
    auto sum = 0;
    aoc::for_each_line(input, [&](const auto line) {
        auto mass = aoc::svatoi(line);
        sum += fuel(mass);
    });
    return sum;
}

template <> auto AocSolution<1, 2019>::part2(const std::string_view &input) const -> int
{
    auto sum = 0;
    aoc::for_each_line(input, [&](const auto line) {
        auto mass = aoc::svatoi(line);
        while (0 < (mass = fuel(mass)))
            sum += mass;
    });
    return sum;
}
