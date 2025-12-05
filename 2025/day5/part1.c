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

    int total = 0;

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

    int ingredients[2000] = {0};

    // Read ingredients
    int ingredients_count = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        int64_t ingredient;
        int result = sscanf(line, "%lld", &ingredient);

        ingredients[ingredients_count] = ingredient;
        ingredients_count += 1;

        // Go through each range
        for (int i = 0; i < range_count; i++)
        {
            if (ranges[i].lower <= ingredient && ingredient <= ranges[i].upper)
            {
                total += 1;
                break;
            }
        }
    }

    printf("%d, %d\n", range_count, ingredients_count);

    printf("Part 1: %d\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}