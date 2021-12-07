#include "aocslns/aocdefs.h"

extern aoc_day_t aoc_2015_day_1;
extern aoc_day_t aoc_2015_day_2;

static const aoc_day_t *const aoc_2015_fns[25] = {&aoc_2015_day_1, &aoc_2015_day_2, 0};

const aoc_day_t *get_2015(int day)
{
    return aoc_2015_fns[day - 1];
}
