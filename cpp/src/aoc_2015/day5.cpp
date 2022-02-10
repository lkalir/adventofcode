#include <algorithm>
#include <string>
#include <string_view>

#include <boost/circular_buffer.hpp>

#include <aoc.h>
#include "util.h"

constexpr int DAY = 5;
constexpr int YEAR = YEAR_2015;

static const std::string day5_name("Doesn't He Have Intern-Elves For This?");

template <> auto AocSolution<DAY, YEAR>::name() const -> const std::string &
{
    return day5_name;
}

static auto is_vowel(const char &c) -> bool
{
    switch (c)
    {
    case 'a':
    case 'e':
    case 'i':
    case 'o':
    case 'u':
        return true;
    default:
        return false;
    }
}

static auto is_nice(const std::string_view &word) -> bool
{
    int vowel_count = 0;
    char last_letter = '\0';
    bool twice_in_a_row = false;
    bool bad_string = false;

    std::for_each(word.begin(), word.end(), [&](const char c) {
        if (is_vowel(c))
            vowel_count++;

        if (last_letter == c)
            twice_in_a_row = true;

        if (('b' == c && last_letter == 'a') || ('d' == c && last_letter == 'c')
            || ('q' == c && last_letter == 'p') || ('y' == c && last_letter == 'x'))
            bad_string = true;

        last_letter = c;
    });

    return (vowel_count >= 3) && twice_in_a_row && !bad_string;
}

static auto is_nice_part_2(const std::string_view &word) -> bool
{
    boost::circular_buffer<char> last_letters(3);
    size_t idx = 0;
    bool split_letters = false;
    bool duplicate_string = false;

    std::for_each(word.begin(), word.end(), [&](const char c) {
        last_letters.push_back(c);

        if (last_letters[0] == last_letters[2])
            split_letters = true;

        if (idx >= 1 && idx <= word.length() - 2 && !duplicate_string)
        {
            const std::string_view letters = word.substr(idx - 1, 2);
            const std::string_view substr = word.substr(idx + 1);

            if (substr.npos != substr.find(letters))
                duplicate_string = true;
        }

        idx++;
    });

    return split_letters && duplicate_string;
}

template <> auto AocSolution<DAY, YEAR>::part1(const std::string_view &input) const -> int
{
    auto cnt = 0;

    aoc::for_each_line(input, [&](auto line) {
        if (is_nice(line))
            cnt++;
    });

    return cnt;
}

template <> auto AocSolution<DAY, YEAR>::part2(const std::string_view &input) const -> int
{
    auto cnt = 0;

    aoc::for_each_line(input, [&](auto line) {
        if (is_nice_part_2(line))
            cnt++;
    });

    return cnt;
}
