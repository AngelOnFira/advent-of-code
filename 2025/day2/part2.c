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

void get_factors(int num, int *factors)
{
    int factors_count = 0;
    for (int i = 1; i < num; i++)
    {
        if (num % i == 0)
        {
            factors[factors_count] = i;
            factors_count += 1;
        }
    }
}

int main(void)
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    printf("hi\n");

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

            // Get the factors of the digit
            int factors[10] = {0};

            get_factors(digits, factors);

            for (int k = 0; k < 10; k++)
            {
                printf("%d\n", factors[k]);
                if (factors[k] == 0)
                {
                    break;
                }
            }
            exit(1);
        }

        i += 1;
    }

    printf("%lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}
