#ifndef AOCSLNS_AOCDEFS_H_
#define AOCSLNS_AOCDEFS_H_

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct
{
    const char *const data;
    size_t len;
} char_view_t;

typedef int (*aoc_fn_t)(const char_view_t *);

typedef struct
{
    const aoc_fn_t part1;
    const aoc_fn_t part2;
    const char *const name;
} aoc_day_t;

#define AOC_DECL(var, p1, p2, challenge) \
    const aoc_day_t var = {.part1 = p1, .part2 = p2, .name = challenge}

#ifdef __cplusplus
}
#endif

#endif
