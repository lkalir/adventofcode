#ifndef __AOC_H__
#define __AOC_H__

#include <stddef.h>

typedef struct
{
    const char *data;
    size_t len;
} char_view_t;

typedef int (*aoc_fn_t)(const char_view_t *);

#endif
