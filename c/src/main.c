#include <adventofcode.h>
#include <aoc.h>
#include <aoc_2015/aoc_2015.h>
#include <getopt.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static const aoc_day_t *const aoc_2015_fns[25] = {&aoc_2015_day_1, &aoc_2015_day_2};
static const aoc_day_t *const aoc_2016_fns[25] = {0};
static const aoc_day_t *const aoc_2017_fns[25] = {0};
static const aoc_day_t *const aoc_2018_fns[25] = {0};
static const aoc_day_t *const aoc_2019_fns[25] = {0};
static const aoc_day_t *const aoc_2020_fns[25] = {0};
static const aoc_day_t *const *const aoc_years[] = {aoc_2015_fns, aoc_2016_fns, aoc_2017_fns,
                                                    aoc_2018_fns, aoc_2019_fns, aoc_2020_fns};

const aoc_day_t *get_day(int year, int day)
{
    if (day < 1 || day > 25)
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

    return aoc_years[y_idx][day - 1];
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
    int day = 0;
    int year = 0;
    const aoc_day_t *day_fns = NULL;

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

    if (NULL == data)
        usage(argv);

    const char_view_t input = {.data = data, .len = strlen(data)};

    if (NULL != (day_fns = get_day(year, day)))
    {
        if ((!part || part == 1) && day_fns->part1)
        {
            int ret = day_fns->part1(&input);
            printf("%d day %d %s part 1: %d\n", year, day, day_fns->name, ret);
        }

        if ((!part || part == 2) && day_fns->part2)
        {
            int ret = day_fns->part2(&input);
            printf("%d day %d %s part 2: %d\n", year, day, day_fns->name, ret);
        }
    }

    return 0;
}
