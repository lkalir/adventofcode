#include <array>
#include <set>
#include <string>
#include <string_view>
#include <utility>

#include <aoc.h>

constexpr int DAY = 3;
constexpr int YEAR = YEAR_2015;

static const std::string day3_name("Perfectly Spherical Houses in a Vacuum");

template <> auto AocSolution<DAY, YEAR>::name() const -> const std::string &
{
    return day3_name;
}

template <> auto AocSolution<DAY, YEAR>::part1(const std::string_view &input) const -> int
{
    std::set<std::pair<int, int>> visited;
    visited.insert(std::pair<int, int>(0, 0));
    int x = 0;
    int y = 0;

    for (auto &c : input)
    {
        switch (c)
        {
        case '^':
            ++y;
            break;
        case '>':
            ++x;
            break;
        case 'v':
            --y;
            break;
        case '<':
            --x;
            break;
        }

        visited.insert(std::pair<int, int>(x, y));
    }

    return (int) visited.size();
}

template <> auto AocSolution<DAY, YEAR>::part2(const std::string_view &input) const -> int
{
    std::set<std::pair<int, int>> visited;
    visited.insert(std::pair<int, int>(0, 0));
    std::array<int, 2> xs = {0, 0};
    std::array<int, 2> ys = {0, 0};
    size_t idx = 0;

    for (auto &c : input)
    {
        switch (c)
        {
        case '^':
            ++ys.at(idx);
            break;
        case '>':
            ++xs.at(idx);
            break;
        case 'v':
            --ys.at(idx);
            break;
        case '<':
            --xs.at(idx);
            break;
        }

        visited.insert(std::pair<int, int>(xs.at(idx), ys.at(idx)));
        idx = (idx + 1) % 2;
    }

    return static_cast<int>(visited.size());
}
