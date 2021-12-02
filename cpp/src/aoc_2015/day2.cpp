#include <algorithm>
#include <aoc.h>
#include <cstdio>
#include <numeric>
#include <string>
#include <string_view>
#include <utility>

#include "util.h"


class Rectangle {
  public:
    Rectangle(int length, int width, int height)
        : m_length(length), m_width(width), m_height(height) {};
    ~Rectangle() = default;

    [[nodiscard]] auto surface_area() const -> int
    {
        return 2 * (m_length * m_width + m_width * m_height + m_length * m_height);
    }

    [[nodiscard]] auto slack() const -> int
    {
        auto dims = this->get_smallest_sides();
        return dims.first * dims.second;
    }

    [[nodiscard]] auto volume() const -> int { return m_length * m_width * m_height; }

    [[nodiscard]] auto bow() const -> int
    {
        auto dims = this->get_smallest_sides();
        return 2 * (dims.first + dims.second);
    }

    static auto from_sv(const std::string_view &input) -> const std::vector<Rectangle>
    {
        std::vector<Rectangle> rects;

        aoc::for_each_line(input, [&](auto line) {
            size_t subpos = 0, subpos2 = 0;
            subpos2 = line.find('x', subpos);
            auto length = aoc::svatoi(line.substr(0, subpos2));
            subpos = line.find('x', subpos2 + 1);
            auto width = aoc::svatoi(line.substr(subpos2 + 1, subpos - subpos2 - 1));
            subpos2 = subpos + 1;
            auto height = aoc::svatoi(line.substr(subpos2, line.length() - subpos2));
            rects.emplace_back(length, width, height);
        });

        return rects;
    }

  private:
    [[nodiscard]] auto get_smallest_sides() const -> std::pair<int, int>
    {
        std::array dims = {m_length, m_width, m_height};
        std::sort(dims.begin(), dims.end());
        return std::make_pair(dims[0], dims[1]);
    }
    const int m_length;
    const int m_width;
    const int m_height;
};

static const std::string day2_name("I Was Told There Would Be No Math");

template <> auto AocSolution<2, 2015>::name() const -> const std::string &
{
    return day2_name;
}

static auto part1(int x, const Rectangle &rect) -> int
{
    return x + rect.surface_area() + rect.slack();
}

static auto part2(int x, const Rectangle &rect) -> int
{
    return x + rect.volume() + rect.bow();
}

template <> auto AocSolution<2, 2015>::part1(const std::string_view &input) const -> int
{
    auto rects = Rectangle::from_sv(input);
    auto sum = 0;
    return std::accumulate(rects.begin(), rects.end(), sum, ::part1);
}

template <> auto AocSolution<2, 2015>::part2(const std::string_view &input) const -> int
{
    auto rects = Rectangle::from_sv(input);
    auto sum = 0;
    return std::accumulate(rects.begin(), rects.end(), sum, ::part2);
}
