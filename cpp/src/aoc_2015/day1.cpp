#include <algorithm>
#include <aoc.h>

static const std::string day1_name("Not Quite Lisp");

template <> auto AocSolution<1, 2015>::name() const -> const std::string &
{
    return day1_name;
}

template <> auto AocSolution<1, 2015>::part1(const std::string_view &input) const -> int
{
    auto up = std::count(input.begin(), input.end(), '(');
    return 2 * up - input.length();
}

template <> auto AocSolution<1, 2015>::part2(const std::string_view &input) const -> int
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
