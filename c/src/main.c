#include "aoc.h"
#include "aoc_2015/aoc_2015.h"
#include <adventofcode.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static const aoc_fn_t aoc_2015_fns[50] = {aoc_2015_day_1_part_1, aoc_2015_day_1_part_2};
static const aoc_fn_t aoc_2016_fns[50] = {0};
static const aoc_fn_t aoc_2017_fns[50] = {0};
static const aoc_fn_t aoc_2018_fns[50] = {0};
static const aoc_fn_t aoc_2019_fns[50] = {0};
static const aoc_fn_t aoc_2020_fns[50] = {0};
static const aoc_fn_t *aoc_years[] = {aoc_2015_fns, aoc_2016_fns, aoc_2017_fns,
                                      aoc_2018_fns, aoc_2019_fns, aoc_2020_fns};

const aoc_fn_t *get_fn(uint16_t year, uint8_t day, int part)
{
    if (!day || day > 25 || !part || part > 2)
    {
        return NULL;
    }

    int y_idx = 0;

    switch (year)
    {
    case 2015:
    case 2016:
    case 2017:
    case 2018:
    case 2019:
    case 2020:
        y_idx = year - 2015;
        break;
    default:
        return NULL;
    }

    return &aoc_years[y_idx][2 * (day - 1) + (part - 1)];
}

void usage(char **argv)
{
    fprintf(stderr, "Usage: %s -d DAY -y YEAR\n", argv[0]);
    exit(EXIT_FAILURE);
}

int main(int argc, char **argv)
{
    int opt = 0;
    int part = 0;
    uint8_t day = 0;
    uint16_t year = 0;
    const aoc_fn_t *fn = NULL;

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
            usage(argv);
        }
    }

    if (day == 0 || year == 0)
        usage(argv);

    const char *data = get_inputs(day, year);
    const char_view_t input = {.data = data, .len = strlen(data)};

    if (!part || part == 1)
    {
        if (NULL != (fn = get_fn(year, day, 1)) && NULL != *fn)
        {
            int ret = (*fn)(&input);
            printf("%d day %d part 1: %d\n", year, day, ret);
        }
    }

    if (!part || part == 2)
    {
        if (NULL != (fn = get_fn(year, day, 2)) && NULL != *fn)
        {
            int ret = (*fn)(&input);
            printf("%d day %d part 2: %d\n", year, day, ret);
        }
    }


    return 0;
}
