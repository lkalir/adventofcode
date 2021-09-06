#ifndef __AOC_H__
#define __AOC_H__

#include <adventofcode.h>
#include <cstdint>
#include <cstring>
#include <string_view>

class AbstractAdventSolution {
  public:
    AbstractAdventSolution(uint8_t day, uint16_t year) : m_day(day), m_year(year) {};
    virtual ~AbstractAdventSolution() = default;

    [[nodiscard]] virtual auto part1(const std::string_view &input) const -> int = 0;
    [[nodiscard]] virtual auto part2(const std::string_view &input) const -> int = 0;
    [[nodiscard]] virtual auto name() const -> const std::string & = 0;

    [[nodiscard]] auto inputs() const -> std::string_view
    {
        const char *data = get_inputs(m_day, m_year);
        return std::string_view(data, strlen(data));
    }

  private:
    const uint8_t m_day;
    const uint16_t m_year;
};

template <int day, int year> class AocSolution : public AbstractAdventSolution {
  public:
    AocSolution() : AbstractAdventSolution(day, year) {}
    ~AocSolution() override = default;
    [[nodiscard]] auto part1(const std::string_view &input) const -> int override;
    [[nodiscard]] auto part2(const std::string_view &input) const -> int override;
    [[nodiscard]] auto name() const -> const std::string & override;
};

#endif
