#include <adventofcode.h>
#include <getopt.h>

#include <iostream>
#include <memory>
#include <span>
#include <stdexcept>

#include <aoc.h>

class AocDate {
  public:
    AocDate(int day, int year) : m_day(day), m_year(year) {};
    ~AocDate() = default;

    [[nodiscard]] auto is_day_valid() const -> bool { return m_day >= 1 && m_day <= 25; }
    [[nodiscard]] auto get_day() const -> int { return m_day; }
    [[nodiscard]] auto get_year() const -> int { return m_year; }

    friend auto operator<<(std::ostream &out, const AocDate &date) -> std::ostream &
    {
        out << date.get_year() << " day " << (int) date.get_day();
        return out;
    }

  private:
    int m_day;
    int m_year;
};

static auto make_2015_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    constexpr int YEAR = YEAR_2015;

    switch (day)
    {
    case 1:
        return std::make_unique<AocSolution<1, YEAR>>();
    case 2:
        return std::make_unique<AocSolution<2, YEAR>>();
    case 3:
        return std::make_unique<AocSolution<3, YEAR>>();
    case 4:
        return std::make_unique<AocSolution<4, YEAR>>();
    case 5:
        return std::make_unique<AocSolution<5, YEAR>>();
    default:
        throw std::range_error("Day not yet implemented");
    }
}

static auto make_2016_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    switch (day)
    {
    default:
        throw std::range_error("Day not yet implemented");
    }
}

static auto make_2017_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    switch (day)
    {
    default:
        throw std::range_error("Day not yet implemented");
    }
}

static auto make_2018_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    switch (day)
    {
    default:
        throw std::range_error("Day not yet implemented");
    }
}

static auto make_2019_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    constexpr int YEAR = YEAR_2019;

    switch (day)
    {
    case 1:
        return std::make_unique<AocSolution<1, YEAR>>();
    default:
        throw std::range_error("Day not yet implemented");
    }
}

static auto make_2020_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    switch (day)
    {
    default:
        throw std::range_error("Day not yet implemented");
    }
}

static auto make_2021_slns(const int day) -> const std::unique_ptr<AbstractAdventSolution>
{
    switch (day)
    {
    default:
        throw std::range_error("Day not yet implemented");
    }
}

auto get_fn(const AocDate &date) -> const std::unique_ptr<AbstractAdventSolution>
{
    if (!date.is_day_valid())
        throw std::invalid_argument("Day must be between 1 and 25");

    switch (date.get_year())
    {
    case YEAR_2015:
        return make_2015_slns(date.get_day());
    case YEAR_2016:
        return make_2016_slns(date.get_day());
    case YEAR_2017:
        return make_2017_slns(date.get_day());
    case YEAR_2018:
        return make_2018_slns(date.get_day());
    case YEAR_2019:
        return make_2019_slns(date.get_day());
    case YEAR_2020:
        return make_2020_slns(date.get_day());
    case YEAR_2021:
        return make_2021_slns(date.get_day());
    default:
        throw std::invalid_argument("Year must be between " + std::to_string(YEAR_START) + " and "
                                    + std::to_string(YEAR_MAX));
    }
}

auto main(int argc, char **argv) -> int
{
    int day = 0;
    int year = 0;
    int opt = 0;
    int part = 0;

    const std::span<char *> args(argv, argc);

    while (-1 != (opt = getopt(argc, argv, "d:y:p:")))
    {
        switch (opt)
        {
        case 'd':
            day = atoi(optarg);
            break;
        case 'y':
            year = atoi(optarg);
            break;
        case 'p':
            part = atoi(optarg);
            break;
        default:
            std::cout << "Usage: " << args.front() << " -d day -y year [-p part]" << std::endl;
            exit(EXIT_FAILURE);
        }
    }

    const AocDate date(day, year);

    try
    {
        const auto fn = get_fn(date);
        if (!part || part == 1)
            std::cout << date << " " << fn->name() << " part 1: " << fn->part1(fn->inputs())
                      << std::endl;

        if (!part || part == 2)
            std::cout << date << " " << fn->name() << " part 2: " << fn->part2(fn->inputs())
                      << std::endl;
    } catch (const std::exception &e)
    {
        std::cerr << year << " day " << (int) day << ": " << e.what() << std::endl;
    }
    return 0;
}
