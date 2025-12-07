#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

const int DIMS = 150;

char filled_space(int x, int y, int *map)
{
    // Handle walls
    if (x < 0 || x >= 140 || y < 0 || y > 140)
    {
        return false;
    }
    return map[x + y * 140] == '@';
}

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

    char map[DIMS * DIMS] = {0};
    char new_map[DIMS * DIMS] = {0};

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
            map[x + y * DIMS] = curr_char;

            if (curr_char == 'S')
            {
                start = x + y * DIMS;
            }

            x += 1;
        }

        y += 1;
    }

    // Put an initial drop below the start
    map[start + DIMS] = '|';

    bool change = true;
    int iters = 0;
    while (change)
    {
        change = false;
        iters += 1;
        printf("%d\n", iters);
        if (iters >= 200) {
            break;
        }

        // Move every | down unless there's ^ or E below it
        for (int i = 0; i < DIMS * DIMS; i++)
        {
            if (map[i] == '|')
            {
                printf("found |\n");
                // Copy self to new_map
                new_map[i] = map[i];

                // If we're at the bottom of the map, continue
                if (i / DIMS >= DIMS)
                {
                    continue;
                }

                // See if we can move below
                char below = map[i + DIMS];
                if (below == '^')
                {
                    total += 1;
                    // Write the new lines coming out
                    new_map[i + DIMS + 1] = '|';
                    new_map[i + DIMS - 1] = '|';
                    new_map[i + DIMS] = 'E';
                    map[i + DIMS] = 'E';
                    change = true;
                }
                else if (below != 'E' && below != '|')
                {
                    new_map[i + DIMS] = '|';
                    change = true;
                }
            } else if (map[i] == 'E' || map[i] == '^') {
                new_map[i] = map[i];
            }
        }

        memcpy(map, new_map, sizeof(map));
        memset(new_map, '0', DIMS * DIMS);

        for (int y = 0; y < DIMS; y++) {
            for (int x = 0; x < DIMS; x++) {
                printf("%c", map[x + y * DIMS]);
            }
            printf("\n");
        }
        printf("\n");
        printf("\n");
        printf("\n");


        if (change == false) {
            break;
        }
    }

    printf("Part 1: %lld\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}