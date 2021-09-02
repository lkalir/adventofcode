#include "aoc_2015.h"

#include <algorithm>
#include <sstream>
#include <string>
#include <tuple>

class Rectangle {
  public:
    Rectangle(int length, int width, int height)
        : m_length(length), m_width(width), m_height(height) {};
    ~Rectangle() = default;

    auto surface_area() -> int
    {
        return 2 * m_length * m_width + 2 * m_width * m_height + 2 * m_length * m_height;
    }

    auto slack() -> int
    {
        auto dims = this->get_smallest_sides();
        return std::get<0>(dims) * std::get<1>(dims);
    }

    auto volume() -> int { return m_length * m_width * m_height; }

    auto bow() -> int
    {
        auto dims = this->get_smallest_sides();
        return 2 * (std::get<0>(dims) + std::get<1>(dims));
    }

  private:
    auto get_smallest_sides() -> std::tuple<int, int>
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
    std::string s(input);
    std::stringstream ss(s);
    std::string token;
    auto up = std::count(input.begin(), input.end(), '(');
    return 2 * up - input.length();
}

auto Aoc2015Day2::part2(const std::string_view &input) const -> int
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
