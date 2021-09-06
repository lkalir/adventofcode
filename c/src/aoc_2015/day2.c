#include <aoc.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
#define SWAP(x, y) \
    {              \
        x = x + y; \
        y = x - y; \
        x = x - y; \
    }

static void smallest(int ret[2], int length, int width, int height)
{
    if (length > height)
        SWAP(length, height);

    if (width > height)
        SWAP(width, height);

    if (length > width)
        SWAP(length, width);

    ret[0] = length;
    ret[1] = width;
}
*/
static int surface_area(int length, int width, int height)
{
    return 2 * (length * width + length * height + width * height);
}

static int volume(int length, int width, int height)
{
    return length * width * height;
}

static int cmpfunc(const void *a, const void *b)
{
    return (*(int *) a - *(int *) b);
}

static int slack(int length, int width, int height)
{
    int ret[3] = {length, width, height};
    // smallest(ret, length, width, height);
    qsort(ret, 3, sizeof(int), cmpfunc);
    return ret[0] * ret[1];
}

static int bow(int length, int width, int height)
{
    int ret[3] = {length, width, height};
    // smallest(ret, length, width, height);
    qsort(ret, 3, sizeof(int), cmpfunc);
    return 2 * (ret[0] + ret[1]);
}

static int aoc_2015_day_2_part_1(const char_view_t *input)
{
    int sum = 0;
    int length = 0;
    int width = 0;
    int height = 0;
    const char *cur = input->data;

    do
    {
        sscanf(cur, "%dx%dx%d", &length, &width, &height);
        sum += surface_area(length, width, height) + slack(length, width, height);
    } while ((const char *) 1
             != (cur = (const char *) memchr(cur + 1, '\n', input->len - (cur - input->data)) + 1));

    return sum;
}

static int aoc_2015_day_2_part_2(const char_view_t *input)
{
    int sum = 0;
    int length = 0;
    int width = 0;
    int height = 0;
    const char *cur = input->data;

    do
    {
        sscanf(cur, "%dx%dx%d", &length, &width, &height);
        sum += volume(length, width, height) + bow(length, width, height);
    } while (NULL != (cur = memchr(cur + 1, '\n', input->len - (cur - input->data))));

    return sum;
}

AOC_DECL(aoc_2015_day_2, aoc_2015_day_2_part_1, aoc_2015_day_2_part_2,
         "I Was Told There Would Be No Math");
