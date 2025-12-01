#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    FILE *fp;
    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("input.txt", "r");
    if (fp == NULL)
        exit(EXIT_FAILURE);

    int counter = 50;
    int multiplier = -1;
    int total = 0;

    while ((read = getline(&line, &len, fp)) != -1)
    {
        int number;
        char direction;

        int result = sscanf(line, "%c%d", &direction, &number);

        if (direction == 'L')
        {
            multiplier = -1;
        }
        else
        {
            multiplier = 1;
        }

        counter = (counter + number * multiplier) % 100;

        if (counter < 0)
        {
            counter = 100 + counter;
        }
        if (counter == 0)
        {
            total += 1;
        }
    }

    printf("Part 1: %d\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}