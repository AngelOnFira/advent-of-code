#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <math.h>
#include <string.h>

int LINES = 4;

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

    char nums[100000];
    memset(nums, 'p', 100000);

    printf("tesT\n");

    // Read ranges
    int line_i = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        if (strcmp(line, "\n") == 0)
        {
            break;
        }

        int i = 0;
        int num_i = 0;
        while (1)
        {
            if (line[i] == '\0')
            {
                break;
            }

            // printf("%c", nums[i + line_i * 5000]);
            nums[i + line_i * 5000] = line[i];

            i += 1;
        }

        line_i += 1;
    }

    bool last_is_add = true;
    int64_t sub_total = 0;
    for (int i = 0; i < 5000; i++)
    {
        char bottom = nums[i + LINES * 5000];

        if (bottom == 'p')
        {
            break;
        }

        // printf("break %c %c %c %c\n", nums[i + 0 * 5000], nums[i + 1 * 5000], nums[i + 2 * 5000], nums[i + 3 * 5000]);

        if (bottom == '*' || bottom == '+')
        {
            last_is_add = bottom == '+';
            total += sub_total;
            // printf("new total %lld %lld\n", sub_total, total);
            sub_total = 0;
        }
        int this_num = 0;
        for (int line = 0; line < LINES; line++)
        {
            char this_char = nums[i + line * 5000];
            int this_digit = this_char - '0';
            if (this_digit >= 0 && this_digit <= 9)
            {
                this_num *= 10;
                this_num += this_digit;
            }
        }
        if (this_num == 0) {
            continue;
        }
        if (last_is_add || sub_total == 0)
        {
            sub_total += this_num;
        }
        else
        {
            sub_total *= this_num;
        }
        // printf("after %d %lld\n", this_num, sub_total);
    }

    total += sub_total;

    printf("Part 2: %lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}