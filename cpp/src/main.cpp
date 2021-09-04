#include <adventofcode.h>
#include <getopt.h>
#include <iostream>
#include <memory>
#include <stdexcept>

#include <aoc.h>
#include <aoc_2015/aoc_2015.h>

static const std::array<std::unique_ptr<AbstractAdventSolution>, 25> aoc_2015_slns = {
    std::make_unique<Aoc2015Day1>(), std::make_unique<Aoc2015Day2>()};
static const std::array<std::unique_ptr<AbstractAdventSolution>, 25> aoc_2016_slns = {};
static const std::array<std::unique_ptr<AbstractAdventSolution>, 25> aoc_2017_slns = {};
static const std::array<std::unique_ptr<AbstractAdventSolution>, 25> aoc_2018_slns = {};
static const std::array<std::unique_ptr<AbstractAdventSolution>, 25> aoc_2019_slns = {};
static const std::array<std::unique_ptr<AbstractAdventSolution>, 25> aoc_2020_slns = {};

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

auto get_fn(const AocDate &date) -> const std::unique_ptr<AbstractAdventSolution> &
{
    if (!date.is_day_valid())
        throw std::invalid_argument("Day must be between 1 and 25");

    const std::array<std::unique_ptr<AbstractAdventSolution>, 25> *sln_list = nullptr;

    switch (date.get_year())
    {
    case 2015:
        sln_list = &aoc_2015_slns;
        break;
    case 2016:
        sln_list = &aoc_2016_slns;
        break;
    case 2017:
        sln_list = &aoc_2017_slns;
        break;
    case 2018:
        sln_list = &aoc_2018_slns;
        break;
    case 2019:
        sln_list = &aoc_2019_slns;
        break;
    case 2020:
        sln_list = &aoc_2020_slns;
        break;
    default:
        throw std::invalid_argument("Year must be between 2015 and 2020");
    }

    auto &ret = sln_list->at(date.get_day() - 1);

    if (!ret)
        throw std::range_error("Day not yet implemented");

    return ret;
}

auto main(int argc, char **argv) -> int
{
    int day = 0;
    int year = 0;
    int opt = 0;

    while (-1 != (opt = getopt(argc, argv, "d:y:")))
    {
        switch (opt)
        {
        case 'd':
            day = atoi(optarg);
            break;
        case 'y':
            year = atoi(optarg);
            break;
        default:
            std::cout << "Usage: " << argv[0] << " -d day -y year" << std::endl;
            exit(EXIT_FAILURE);
        }
    }

    const AocDate date(day, year);

    try
    {
        auto &fn = get_fn(date);
        std::cout << date << " " << fn->name() << " part 1: " << fn->part1(fn->inputs())
                  << std::endl;
        std::cout << date << " " << fn->name() << " part 2: " << fn->part2(fn->inputs())
                  << std::endl;
    } catch (const std::exception &e)
    {
        std::cerr << year << " day " << (int) day << ": " << e.what() << std::endl;
    }
    return 0;
}
