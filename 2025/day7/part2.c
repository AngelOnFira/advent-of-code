#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

const int DIMS = 150;

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

    int64_t map[DIMS * DIMS] = {0};
    int64_t new_map[DIMS * DIMS] = {0};

    // Read ranges
    int y = 0;
    int start = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        char curr_char;
        int x = 0;
        while (1)
        {
            curr_char = line[x];
            if (curr_char == '\0' || curr_char == '\n')
            {
                break;
            }
            if (curr_char == '^')
            {
                map[x + y * DIMS] = -1;
            }

            if (curr_char == 'S')
            {
                start = x + y * DIMS;
            }

            x += 1;
        }

        y += 1;
    }

    // Put an initial drop below the start
    map[start + DIMS] = 1;

    bool change = true;
    int iters = 0;
    while (change)
    {
        change = false;
        iters += 1;
        printf("%d\n", iters);
        if (iters >= 160)
        {
            break;
        }

        // Move every | down unless there's ^ or E below it
        for (int i = 0; i < DIMS * DIMS; i++)
        {
            if (map[i] > 0)
            {
                // printf("found |\n");
                // Copy self to new_map
                new_map[i] += map[i];

                // If we're at the bottom of the map, continue
                if (i / DIMS >= DIMS)
                {
                    continue;
                }

                // See if we can move below
                int64_t below = map[i + DIMS];
                if (below == -1)
                {
                    // Write the new lines coming out
                    new_map[i + DIMS + 1] += map[i];
                    new_map[i + DIMS - 1] += map[i];
                    map[i + DIMS] = -2;
                    new_map[i + DIMS] = -2;
                    change = true;
                }
                else if (below == 0)
                {
                    new_map[i + DIMS] += map[i];
                    change = true;
                }
            }
            else if (map[i] < 0 && map[i] >= -2)
            {
                new_map[i] = map[i];
            }
        }

        for (int y = 0; y < DIMS; y++)
        {
            for (int x = 0; x < DIMS; x++)
            {
                printf("%16lld ", new_map[x + y * DIMS]);
            }
            printf("\n");
        }
        printf("\n");
        printf("\n");
        printf("\n");

        memcpy(map, new_map, sizeof(map));
        memset(new_map, 0, sizeof(new_map));

        if (change == false)
        {
            break;
        }
    }

    // Add up all the nums on the final line
    for (int i = 0; i < DIMS; i++)
    {
        total += map[i + DIMS * (DIMS - 1)];
    }

    printf("Part 2: %lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}