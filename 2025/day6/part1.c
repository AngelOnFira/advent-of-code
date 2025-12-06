#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

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

    int64_t nums[5000] = {0};

    printf("tesT\n");

    // Read ranges
    int line_i = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        if (strcmp(line, "\n") == 0)
        {
            break;
        }

        printf("tesT\n");

        // The final line
        if (line[0] == '+' || line[0] == '*')
        {
            // Go through each element
            int i = 0;
            int num_i = 0;
            while (1)
            {
                // printf("%c", line[i]);
                if (line[i] == ' ')
                {
                    i += 1;
                    continue;
                }
                else if (line[i] == '*')
                {
                    total += nums[num_i] * nums[num_i + 1000] * nums[num_i + 2000] * nums[num_i + 3000];
                    num_i += 1;
                }
                else if (line[i] == '+')
                {
                    total += nums[num_i] + nums[num_i + 1000] + nums[num_i + 2000] + nums[num_i + 3000];
                    num_i += 1;
                }
                else
                {
                    break;
                }
                i += 1;
            }
        }
        else
        {
            // Go through each element
            int curr_num = 0;
            int i = 0;
            int num_i = 0;
            while (1)
            {
                printf("%c", line[i]);
                if (line[i] == ' ')
                {
                    if (curr_num != 0)
                    {
                        printf("accessing %d\n", num_i + line_i * 1000);
                        printf("with %d %d\n", num_i, line_i);
                        nums[num_i + line_i * 1000] = curr_num;
                        curr_num = 0;
                        num_i += 1;
                    }
                }
                else if (line[i] >= '0' && line[i] <= '9')
                {
                    curr_num *= 10;
                    curr_num += line[i] - '0';
                }
                else
                {
                    nums[num_i + line_i * 1000] = curr_num;
                    break;
                }
                i += 1;
            }
            line_i += 1;
        }
    }

    printf("Part 1: %lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}