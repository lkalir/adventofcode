#ifndef __AOC_2015_H__
#define __AOC_2015_H__

#include <adventofcode.h>
#include <aoc.h>

class Aoc2015Day1 : public AbstractAdventSolution {
  public:
    Aoc2015Day1() : AbstractAdventSolution(1, 2015) {}
    ~Aoc2015Day1() = default;

    [[nodiscard]] auto part1(const std::string_view &input) const -> int override;
    [[nodiscard]] auto part2(const std::string_view &input) const -> int override;
    [[nodiscard]] auto name() const -> const std::string & override;
};

class Aoc2015Day2 : public AbstractAdventSolution {
  public:
    Aoc2015Day2() : AbstractAdventSolution(2, 2015) {}
    ~Aoc2015Day2() = default;

    [[nodiscard]] auto part1(const std::string_view &input) const -> int override;
    [[nodiscard]] auto part2(const std::string_view &input) const -> int override;
    [[nodiscard]] auto name() const -> const std::string & override;
};

#endif
