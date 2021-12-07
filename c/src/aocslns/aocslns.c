#include "aocslns/aoc_2015/aoc_2015.h"
#include "aocslns/aocdefs.h"

const aoc_day_t *get_sln(int year, int day)
{
    if (day < 1 || day > 25)
    {
        return NULL;
    }

    switch (year)
    {
    case 2015:
        return get_2015(day);
    case 2016:
    case 2017:
    case 2018:
    case 2019:
    case 2020:
    case 2021:
    default:
        return NULL;
    }
}
