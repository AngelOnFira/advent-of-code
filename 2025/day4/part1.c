#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

bool filled_space(int x, int y, int *map)
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

    int total = 0;

    int map[140 * 140] = {0};

    int y = 0;
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
            map[x + y * 140] = curr_char;
            x += 1;
        }

        y += 1;
    }

    // Go through each, check the nearby ones, add one if it's <4
    for (int i = 0; i < 140 * 140; i++)
    {
        // printf("checking %d %d\n", i % 140, i / 140);

        // If it's not filled, skip
        if (!filled_space(i % 140, i / 140, map))
        {
            continue;
        }

        int count = 0;
        for (int y = -1; y <= 1; y++)
        {
            for (int x = -1; x <= 1; x++)
            {
                if (x == 0 && y == 0)
                {
                    continue;
                }

                if (filled_space(x + i % 140, y + i / 140, map))
                {
                    // printf("%d %d\n", x + i % 140, y + i / 140);
                    count += 1;
                }
            }
        }
        // printf("%d\n", count);
        // printf("\n\n\n");
        if (count < 4)
        {
            total += 1;
        }
    }

    printf("Part 1: %d\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}