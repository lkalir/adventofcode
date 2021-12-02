#include <functional>
#include <string_view>
#include <vector>

namespace aoc {
auto for_each_line(const std::string_view &lines, const std::function<void(std::string_view &)> &F)
    -> void
{
    size_t pos = 0, pos2 = 0;

    while (lines.npos != (pos2 = lines.find('\n', pos)))
    {
        auto line = lines.substr(pos, pos2 - pos);
        F(line);
        pos = pos2 + 1;
    }
}

/// Technically atoi should really be astoin, therefore, this should thus be stviastoin
auto svatoi(const std::string_view &a) -> int
{
    auto res = 0;

    for (auto &c : a)
    {
        int d = static_cast<int>(c) - '0';
        res = res * 10 + d;
    }

    return res;
}

} // namespace aoc
