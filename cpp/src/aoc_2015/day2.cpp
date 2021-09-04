#include "aoc_2015.h"

#include <algorithm>
#include <cstdio>
#include <iostream>
#include <string>
#include <string_view>
#include <tuple>

/// Technically atoi should really be astoin, therefore, this should thus be stviastoin
static auto svatoi(const std::string_view &a) -> int
{
    auto res = 0;

    for (auto &c : a)
    {
        int d = static_cast<int>(c) - '0';
        res = res * 10 + d;
    }

    return res;
}

class Rectangle {
  public:
    Rectangle(int length, int width, int height)
        : m_length(length), m_width(width), m_height(height) {};
    ~Rectangle() = default;

    [[nodiscard]] auto surface_area() const -> int
    {
        return 2 * m_length * m_width + 2 * m_width * m_height + 2 * m_length * m_height;
    }

    [[nodiscard]] auto slack() const -> int
    {
        auto dims = this->get_smallest_sides();
        return std::get<0>(dims) * std::get<1>(dims);
    }

    [[nodiscard]] auto volume() const -> int { return m_length * m_width * m_height; }

    [[nodiscard]] auto bow() const -> int
    {
        auto dims = this->get_smallest_sides();
        return 2 * (std::get<0>(dims) + std::get<1>(dims));
    }

    static auto from_sv(const std::string_view &input) -> const std::vector<Rectangle>
    {
        std::vector<Rectangle> rects;
        size_t pos = 0, pos2 = 0;

        while (input.npos != (pos2 = input.find('\n', pos)))
        {
            auto line = input.substr(pos, pos2 - pos);
            size_t subpos = 0, subpos2 = 0;
            subpos2 = line.find('x', subpos);
            auto length = svatoi(line.substr(0, subpos2));
            subpos = line.find('x', subpos2 + 1);
            auto width = svatoi(line.substr(subpos2 + 1, subpos - subpos2 - 1));
            subpos2 = subpos + 1;
            auto height = svatoi(line.substr(subpos2, line.length() - subpos2));
            rects.emplace_back(length, width, height);
            pos = pos2 + 1;
        }

        return rects;
    }

  private:
    [[nodiscard]] auto get_smallest_sides() const -> std::tuple<int, int>
    {
        std::array dims = {m_length, m_width, m_height};
        std::sort(dims.begin(), dims.end());
        return std::make_tuple(dims[0], dims[1]);
    }
    const int m_length;
    const int m_width;
    const int m_height;
};

static const std::string day2_name("I Was Told There Would Be No Math");

auto Aoc2015Day2::name() const -> const std::string &
{
    return day2_name;
}

auto Aoc2015Day2::part1(const std::string_view &input) const -> int
{
    auto rects = Rectangle::from_sv(input);
    auto sum = 0;
    std::for_each(rects.begin(), rects.end(),
                  [&sum](const Rectangle &rect) { sum += rect.surface_area() + rect.slack(); });
    return sum;
}

auto Aoc2015Day2::part2(const std::string_view &input) const -> int
{
    auto rects = Rectangle::from_sv(input);
    auto sum = 0;
    std::for_each(rects.begin(), rects.end(),
                  [&sum](const Rectangle &rect) { sum += rect.volume() + rect.bow(); });
    return sum;
}
