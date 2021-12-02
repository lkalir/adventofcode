#ifndef __AOC_UTIL_H__
#define __AOC_UTIL_H__

#include <functional>
#include <string_view>

namespace aoc {
auto for_each_line(const std::string_view &lines, const std::function<void(std::string_view &)> &F)
    -> void;

auto svatoi(const std::string_view &a) -> int;
} // namespace aoc
#endif
