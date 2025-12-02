#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

struct Range
{
    int lower;
    int upper;
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

    getline(&line, &len, fp);

    // printf(line);

    char *range = strtok(line, ",");
    char *ranges[10000];
    ranges[0] = range;
    int i = 1;

    while (range != NULL)
    {
        // printf("%s\n", range);
        range = strtok(NULL, ",");
        ranges[i] = range;
        i += 1;
    }

    i = 0;

    int64_t total = 0;

    while (ranges[i] != NULL)
    {
        int64_t lower = atoll(strtok(ranges[i], "-"));
        int64_t upper = atoll(strtok(NULL, "-"));
        // printf("%lld\n", lower);
        // printf("%lld\n", upper);

        for (int64_t j = lower; j <= upper; j++)
        {
            int digits = floor(log10(j)) + 1;
            if (digits % 2 == 0)
            {
                // printf("%lld\n", j);

                int first = j / (int)(pow(10, digits / 2));
                int second = j % (int)(pow(10, digits / 2));
                
                if (first == second)
                {
                    printf("%d\n", first);
                    // printf("%d\n", second);
                    total += j;
                }
                // exit(1);
            }
        }

        i += 1;
    }

    printf("%lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}