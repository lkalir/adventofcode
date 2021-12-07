#include <adventofcode.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "aocslns/aocdefs.h"
#include "aocslns/aocslns.h"

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

    if (NULL != (day_fns = get_sln(year, day)))
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
