#ifndef __AOC_H__
#define __AOC_H__

#include <stddef.h>

typedef struct
{
    const char *data;
    size_t len;
} char_view_t;

typedef int (*aoc_fn_t)(const char_view_t *);

typedef struct
{
    const aoc_fn_t part1;
    const aoc_fn_t part2;
    const char *name;
} aoc_day_t;

#define AOC_DECL(var, p1, p2, challenge) \
    const aoc_day_t var = {.part1 = p1, .part2 = p2, .name = challenge}

#endif
