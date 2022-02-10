#include <array>
#include <cstring>
#include <iomanip>
#include <memory>
#include <string>
#include <string_view>

#include <openssl/md5.h>

#include <aoc.h>

constexpr int DAY = 4;
constexpr int YEAR = YEAR_2015;

static const std::string day4_name("The Ideal Stocking Stuffer");

template <> auto AocSolution<DAY, YEAR>::name() const -> const std::string &
{
    return day4_name;
}

template <> auto AocSolution<DAY, YEAR>::part1(const std::string_view &input) const -> int
{
    auto inbuf = std::unique_ptr<char[]>(new char[input.length() + 10]);
    input.copy(&inbuf[0], input.length() - 1);
    std::array<unsigned char, MD5_DIGEST_LENGTH> md5buf {};
    int i = 0;

    for (i = 1; i <= INT_MAX; ++i)
    {
        std::ostringstream foo;
        foo << i;
        foo.str().copy(&inbuf[input.length() - 1], foo.str().length());
        MD5(reinterpret_cast<unsigned char *>(inbuf.get()), input.length() + foo.str().length() - 1,
            md5buf.data());

        if (md5buf[0] == 0 && md5buf[1] == 0 && (md5buf[2] & 0xF0) == 0)
            return i;
    }

    return i;
}

template <> auto AocSolution<DAY, YEAR>::part2(const std::string_view &input) const -> int
{
    auto inbuf = std::unique_ptr<char[]>(new char[input.length() + 10]);
    input.copy(&inbuf[0], input.length() - 1);
    std::array<unsigned char, MD5_DIGEST_LENGTH> md5buf {};
    int i = 0;

    for (i = 1; i <= INT_MAX; ++i)
    {
        std::ostringstream foo;
        foo << i;
        foo.str().copy(&inbuf[input.length() - 1], foo.str().length());
        MD5(reinterpret_cast<unsigned char *>(inbuf.get()), input.length() + foo.str().length() - 1,
            md5buf.data());

        if (md5buf[0] == 0 && md5buf[1] == 0 && md5buf[2] == 0)
            return i;
    }

    return i;
}
