#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

const int COUNT = 1000;

struct Point
{
    int x;
    int y;
    int z;
};

struct Pair
{
    struct Point first;
    struct Point second;
    int distance;
};

bool is_point_same(struct Point p1, struct Point p2)
{
    return (p1.x == p2.x && p1.y == p2.y && p1.z == p2.z);
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
    int total_stored_pairs = 0;

    struct Point points[COUNT * 10];
    struct Pair sorted_pairs[COUNT * 10];

    // Read ranges
    int input_line_count = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        int x, y, z;
        int result = sscanf(line, "%d,%d,%d", &x, &y, &z);

        struct Point point = {x, y, z};
        points[input_line_count] = point;

        input_line_count += 1;
    }

    int check_counter = 0;

    // n = 1000, QED bubble sort
    for (int top_count = 0; top_count < COUNT; top_count++)
    {
        struct Pair min_pair;
        bool found_min_pair = false;

        // Go through each pair
        for (int i1 = 0; i1 < input_line_count; i1++)
        {
            for (int i2 = 0; i2 < input_line_count; i2++)
            {
                if (i1 == i2)
                {
                    continue;
                }

                check_counter += 1;

                // Get the distance
                int dist_sqrt = pow(points[i1].x - points[i2].x, 2) + pow(points[i1].y - points[i2].y, 2) + pow(points[i1].z - points[i2].z, 2);
                if (!found_min_pair || dist_sqrt < min_pair.distance)
                {
                    // Check if this pair is already in the list
                    bool found = false;
                    for (int min_pair_i = 0; min_pair_i < total_stored_pairs; min_pair_i++)
                    {
                        // We're going to assume that distances between all
                        // pairs are unique because the question leaves that
                        // ambiguous
                        int check_dist = sorted_pairs[min_pair_i].distance;
                        if (check_dist == dist_sqrt)
                        {
                            // It exists
                            found = true;
                            break;
                        }
                    }

                    // If we found the element, continue to the next pair
                    if (found)
                    {
                        continue;
                    }

                    // Otherwise, change the min point
                    struct Point new_point_1 =
                        {
                            points[i1].x,
                            points[i1].y,
                            points[i1].z};
                    struct Point new_point_2 =
                        {
                            points[i2].x,
                            points[i2].y,
                            points[i2].z};
                    struct Pair new_min_pair = {new_point_1, new_point_2, dist_sqrt};

                    min_pair = new_min_pair;
                    found_min_pair = true;
                    // printf("seeting main pair %d\n", dist_sqrt);
                }
            }
        }

        sorted_pairs[total_stored_pairs] = min_pair;
        total_stored_pairs += 1;
    }

    for (int i = 0; i < 5; i++)
    {
        printf("Selected %d %d, %d, %d | %d, %d, %d\n", sorted_pairs[i].distance, sorted_pairs[i].first.x, sorted_pairs[i].first.y, sorted_pairs[i].first.z, sorted_pairs[i].second.x, sorted_pairs[i].second.y, sorted_pairs[i].second.z);
    }

    // Now we meed to find the largest connected graph
    struct Point already_checked[COUNT * 10];
    int already_checked_i = 0;
    int top_three[3] = {0};

    // We need to repeat until we don't add anything else to the list, because
    // on the first pass it might not add some correctly

    for (int outer = 0; outer < COUNT; outer++)
    {
        struct Point this_round_checked[COUNT * 19];
        int this_round_checked_i = 0;

        while (1)
        {
            int this_round_checked_start = this_round_checked_i;

            for (int i = outer; i < COUNT; i++)
            {
                // Look at both pairs, if either are in the already checked, we can
                // continue because we would have looked at both before
                bool found = false;
                for (int ii = 0; ii < already_checked_i; ii++)
                {
                    if (is_point_same(already_checked[ii], sorted_pairs[i].first) && is_point_same(already_checked[ii], sorted_pairs[i].second))
                    {
                        found = true;
                        break;
                    }
                }
                // If we found one, we can just to go the next
                if (found)
                {
                    continue;
                }

                // If the this_round_checked is empty, add both to it
                if (this_round_checked_i == 0)
                {
                    this_round_checked[this_round_checked_i] = sorted_pairs[i].first;
                    this_round_checked[this_round_checked_i + 1] = sorted_pairs[i].second;
                    this_round_checked_i += 2;

                    // And add them to already checked
                    already_checked[already_checked_i] = sorted_pairs[i].first;
                    already_checked[already_checked_i + 1] = sorted_pairs[i].second;
                    already_checked_i += 2;

                    printf("Adding %d, %d, %d\n", sorted_pairs[i].first.x, sorted_pairs[i].first.y, sorted_pairs[i].first.z);
                    printf("Adding %d, %d, %d\n", sorted_pairs[i].second.x, sorted_pairs[i].second.y, sorted_pairs[i].second.z);
                }
                // Otherwise, see if the first and second are already in the list.
                // if either is, add the other to the list
                else
                {
                    bool found_first = false;
                    bool found_second = false;
                    for (int ii = 0; ii < already_checked_i; ii++)
                    {
                        if (is_point_same(already_checked[ii], sorted_pairs[i].first))
                        {
                            found_first = true;
                        }

                        if (is_point_same(already_checked[ii], sorted_pairs[i].second))
                        {
                            found_second = true;
                        }

                        // This will happen if both are already in the same circuit
                        // already, so we won't trigger the next checks
                        if (found_first && found_second)
                        {
                            break;
                        }
                    }

                    if (found_first && !found_second)
                    {
                        already_checked[already_checked_i++] = sorted_pairs[i].second;
                        this_round_checked[this_round_checked_i++] = sorted_pairs[i].second;
                        printf("Adding %d, %d, %d\n", sorted_pairs[i].second.x, sorted_pairs[i].second.y, sorted_pairs[i].second.z);
                    }

                    if (found_second && !found_first)
                    {
                        already_checked[already_checked_i++] = sorted_pairs[i].first;
                        this_round_checked[this_round_checked_i++] = sorted_pairs[i].first;
                        printf("Adding %d, %d, %d\n", sorted_pairs[i].first.x, sorted_pairs[i].first.y, sorted_pairs[i].first.z);
                    }
                }
            }

            printf("\n");

            // If this_round_checked_start is the same, we can finish this loop
            if (this_round_checked_start == this_round_checked_i)
            {
                break;
            }
        }

        if (this_round_checked_i > top_three[0])
        {
            top_three[2] = top_three[1];
            top_three[1] = top_three[0];
            top_three[0] = this_round_checked_i;
        }
        else if (this_round_checked_i > top_three[1])
        {
            top_three[2] = top_three[1];
            top_three[1] = this_round_checked_i;
        }
        else if (this_round_checked_i > top_three[2])
        {
            top_three[2] = this_round_checked_i;
        }
    }

    printf("%d %d %d\n", top_three[0], top_three[1], top_three[2]);
    printf("Part 1: %d\n", top_three[0] * top_three[1] * top_three[2]);
    // printf("check counter 2: %d\n", check_counter);

    free(line);
    exit(EXIT_SUCCESS);
}