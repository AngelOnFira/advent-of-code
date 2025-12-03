#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>

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

void get_digits(int64_t num, int digit_count, int *digits)
{
    for (int i = 0; i < digit_count; i++)
    {
        digits[i] = num / pow(10, digit_count - 1 - i);
        num %= (int)pow(10, digit_count - 1 - i);
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
            int digit_count = floor(log10(j)) + 1;
            // printf("%d", j);

            // Get the factors of the digit
            int factors[10] = {0};

            get_factors(digit_count, factors);

            int digits[30] = {0};

            // Get the integer digits
            get_digits(j, digit_count, digits);

            // printf("%lld\n", j);

            bool success_found = false;

            for (int k = 0; k < 10; k++)
            {
                // Go through each factor
                int factor_i = 0;
                while (1)
                {
                    if (factors[factor_i] == 0)
                    {
                        break;
                    }

                    // printf("%d", factors[factor_i]);

                    int64_t last_num = 0;
                    bool invalid_factor = false;

                    // for (int g = 0; g < factors[factor_i]; g++)
                    // {
                    // Get the first n numbers of the digits, and combine
                    // them
                    int64_t current_count = 0;
                    for (int ii = 0; ii < digit_count; ii++)
                    {
                        current_count *= 10;
                        current_count += digits[ii];

                        // If we're at the edge of a factor, we can check
                        // against the last stored number
                        if (ii + 1 % factors[factor_i] == 0)
                        {
                            if (last_num == 0)
                            {
                                last_num = current_count;
                            }
                            // If they don't match, then we're looking at a
                            // breakdown that doesn't work
                            if (last_num != current_count)
                            {
                                invalid_factor = true;
                                break;
                            }
                            last_num = current_count;
                            current_count = 0;
                        }
                    }

                    // If we haven't found this factor to be invalid yet,
                    // then we've found a valid one! Add it to the total
                    if (!invalid_factor)
                    {
                        // printf("found %lld with factor %d and digits %d\n", j, factors[factor_i], digit_count);
                        total += j;
                        success_found = true;

                        // We need to break so we don't keep checking
                        // factors
                        break;
                    }
                    // }

                    factor_i += 1;
                }

                if (success_found) {
                    break;
                }
            }
        }

        i += 1;
    }

    printf("%lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}
