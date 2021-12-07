#include <stddef.h>

#include "aocslns/aocdefs.h"

static int aoc_2015_day_1_part_1(const char_view_t *input)
{
    int sum = 0;

    for (size_t i = 0; i < input->len; i++)
    {
        if (input->data[i] == '(')
            sum++;
    }

    return 2 * sum - input->len;
}

static int aoc_2015_day_1_part_2(const char_view_t *input)
{
    int floor = 0;

    for (size_t i = 0; i < input->len; i++)
    {
        if (input->data[i] == '(')
            ++floor;
        else
            --floor;

        if (floor < 0)
            return i + 1;
    }

    return -1;
}

AOC_DECL(aoc_2015_day_1, aoc_2015_day_1_part_1, aoc_2015_day_1_part_2, "Not Quite Lisp");
