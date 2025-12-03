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
        char curr_char;
        int i = 0;
        int line_nums[100] = {0};
        while (1)
        {
            curr_char = line[i];
            if (curr_char == '\0' || curr_char == '\n')
            {
                break;
            }
            line_nums[i] = curr_char - '0';
            // printf("%d", line_nums[i]);
            i += 1;
        }
        // printf("\n");
        // Iterate over and find the first largest digit
        int largest = 0;
        int largest_i = 0;
        for (int j = 0; j < i - 1; j++) {
            if (line_nums[j] > largest) {
                largest = line_nums[j];
                largest_i = j;
            }
        }

        // Find the second number
        int largest2 = 0;
        for (int j = largest_i + 1; j < i; j++) {
            if (line_nums[j] > largest2) {
                largest2 = line_nums[j];
            }
        }

        int result = largest * 10 + largest2;
        // printf("%d\n", result);
        total += result;
    }

    printf("Part 1: %d\n", total);

    free(line);
    exit(EXIT_SUCCESS);
}