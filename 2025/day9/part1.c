#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

struct Point
{
    int64_t x;
    int64_t y;
};

int main(void)
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("input.txt", "r");
    if (fp == NULL)
        exit(EXIT_FAILURE);

    int total = 0;
    int total_stored_pairs = 0;

    struct Point points[10000];

    // Read ranges
    int dims = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        int x, y, z;
        int result = sscanf(line, "%d,%d", &x, &y);

        struct Point point = {x, y};
        points[dims] = point;

        dims += 1;
    }

    // Find the lowest top left and highest bottom right
    int64_t max_area = 0;
    for (int i1 = 0; i1 < dims; i1++)
    {
        for (int i2 = 0; i2 < dims; i2++)
        {
            if (i1 == i2)
            {
                continue;
            }

            int64_t area = (llabs(points[i1].x - points[i2].x) + 1) * (llabs(points[i1].y - points[i2].y) + 1);
            if (area > max_area)
            {
                max_area = area;
                // printf("found on %d %d , %d %d\n", points[i1].x, points[i1].y, points[i2].x, points[i2].y);
            }
        }
    }

    printf("Part 1: %lld\n", max_area);
    // printf("check counter 2: %d\n", check_counter);

    free(line);
    exit(EXIT_SUCCESS);
}