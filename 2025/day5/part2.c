#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

struct Range
{
    int64_t lower;
    int64_t upper;
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

    int64_t total = 0;

    struct Range ranges[400];

    // Read ranges
    int range_count = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        if (strcmp(line, "\n") == 0)
        {
            break;
        }

        int64_t lower;
        int64_t upper;
        int result = sscanf(line, "%lld-%lld", &lower, &upper);

        struct Range range = {lower, upper};
        ranges[range_count] = range;
        range_count += 1;
    }

    // Check each range against each other, reduce ranges that overlap
    for (int range_1 = 0; range_1 < range_count; range_1++)
    {
        for (int range_2 = 0; range_2 < range_count; range_2++)
        {
            if (range_1 == range_2)
            {
                continue;
            }

            // Reduce overlapping ranges
            if (ranges[range_1].upper >= ranges[range_2].lower && ranges[range_1].upper <= ranges[range_2].upper)
            {
                if (ranges[range_1].lower >= ranges[range_2].lower)
                {
                    // Nuke it
                    ranges[range_1].lower = 0;
                    ranges[range_1].upper = 0;
                }
                else
                {
                    // Truncate it
                    ranges[range_1].upper = ranges[range_2].lower - 1;
                }
            }

            if (ranges[range_2].upper >= ranges[range_1].lower && ranges[range_2].upper <= ranges[range_1].upper)
            {
                if (ranges[range_2].lower >= ranges[range_1].lower)
                {
                    // Nuke it
                    ranges[range_2].lower = 0;
                    ranges[range_2].upper = 0;
                }
                else
                {
                    // Truncate it
                    ranges[range_2].upper = ranges[range_1].lower - 1;
                }
            }

            if (ranges[range_1].lower >= ranges[range_2].upper && ranges[range_1].lower <= ranges[range_2].lower)
            {
                if (ranges[range_1].upper >= ranges[range_2].upper)
                {
                    // Nuke it
                    ranges[range_1].upper = 0;
                    ranges[range_1].lower = 0;
                }
                else
                {
                    // Truncate it
                    ranges[range_1].lower = ranges[range_2].upper + 1;
                }
            }

            if (ranges[range_2].lower >= ranges[range_1].upper && ranges[range_2].lower <= ranges[range_1].lower)
            {
                if (ranges[range_2].upper >= ranges[range_1].upper)
                {
                    // Nuke it
                    ranges[range_2].upper = 0;
                    ranges[range_2].lower = 0;
                }
                else
                {
                    // Truncate it
                    ranges[range_2].lower = ranges[range_1].upper + 1;
                }
            }
        }
    }

    // Pass over them again to add them up
    for (int range_1 = 0; range_1 < range_count; range_1++)
    {
        total += ranges[range_1].upper - ranges[range_1].lower;
        // printf("%lld %lld\n", ranges[range_1].lower, ranges[range_1].upper);

        if (ranges[range_1].upper != 0 || ranges[range_1].lower != 0) {
            total += 1;
        }
    }

    printf("Part 1: %lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}