#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <regex.h>

#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))

static const char *const RE_PATTERN = "^\\[([.#]+)\\]";
static const char *const RE_PARENS = "\\(([0-9,]+)\\)";
static const char *const RE_CURLY = "\\{([0-9,]+)\\}";

int recurse(
    // Initial joltages
    int *joltages,
    // Initial buttons
    uint16_t *buttons[30],
    // How far into the button list we are
    int current_button,
    // What's already been pressed, and how many times
    int *current_solve[30])
{
    // Make a copy of joltages
    int new_joltages[20];
    int new_current_solve[20];

    memcpy(new_joltages, joltages, sizeof(new_joltages));
    memcpy(new_current_solve, current_solve, sizeof(new_current_solve));

    // Check if the current solve works

    // If we can reduce with the current button more, we should do that
    int best_solve = recurse(new_joltages);

    // If best solve is > 0, we've got the solution
    if (best_solve > 0)
    {
        return best_solve;
    }

    // Ok, we've tried that, and we haven't found a solve yet. Go to the next
    // largest button, unless there are multiple of the same size.

    int best_solve = recurse(new_joltages);
}

int recurion_init()
{
    int joltages[20];

    int best_solve = recurse(joltages);
}

void print_binary16(uint16_t x)
{
    for (int i = 15; i >= 0; i--)
    {
        printf("%d", (x >> i) & 1);
    }
    printf("\n");
}

struct Machine
{
    uint16_t state;
    uint16_t buttons[30];
    int button_count;
    int joltages[20];
    int joltage_count;
};

int main(void)
{
    FILE *fp = fopen("input.txt", "r");
    if (!fp)
    {
        perror("input.txt");
        return 1;
    }

    char *line = NULL;
    size_t len = 0;
    ssize_t read;

    int total = 0;

    struct Machine machines[200];

    regex_t re_pat, re_parens, re_curly;

    regcomp(&re_pat, RE_PATTERN, REG_EXTENDED);
    regcomp(&re_parens, RE_PARENS, REG_EXTENDED);
    regcomp(&re_curly, RE_CURLY, REG_EXTENDED);

    int line_count = 0;
    while ((read = getline(&line, &len, fp)) != -1)
    {
        // printf("LINE: %s", line);

        uint16_t state = 0;
        uint16_t buttons[15];
        int joltages[20];
        int joltage_count = 0;

        regmatch_t mpat[2];
        if (regexec(&re_pat, line, ARRAY_SIZE(mpat), mpat, 0) == 0)
        {
            int plen = mpat[1].rm_eo - mpat[1].rm_so;

            for (int i = 0; i < 16 - 1; i++)
            {
                char test = *(line + mpat[1].rm_so + i);
                // printf("%c\n", test);
                if (test == '#')
                {
                    state |= 0b0000000000000001;
                }
                // This should also work for things after the [#.] because "it's
                // fine" and "I'm a trusted developer"
                state <<= 1;
            }
            // print_binary16(state);
        }

        regmatch_t mcurly[2];
        if (regexec(&re_curly, line, ARRAY_SIZE(mcurly), mcurly, 0) == 0)
        {
            int clen = mcurly[1].rm_eo - mcurly[1].rm_so;
            printf("Curly:   \"%.*s\"\n", clen, line + mcurly[1].rm_so);

            char buf[64];
            memcpy(buf, line + mcurly[1].rm_so, clen);
            buf[len] = '\0';

            char *tok = strtok(buf, ",");
            int counter = 0;
            while (tok != NULL)
            {
                int value = atoi(tok);

                joltages[joltage_count++] = value;
            }
        }

        // printf("Parens:\n");
        char *s = line;
        regmatch_t mparen[2];
        unsigned int button_count = 0;

        uint16_t this_button = 0;
        while (regexec(&re_parens, s, ARRAY_SIZE(mparen), mparen, 0) == 0)
        {
            int len = mparen[1].rm_eo - mparen[1].rm_so;
            // printf("  #%u: \"%.*s\"\n", i, len, s + mparen[1].rm_so);

            char buf[64];
            memcpy(buf, s + mparen[1].rm_so, len);
            buf[len] = '\0';

            char *tok = strtok(buf, ",");
            int counter = 0;
            while (tok != NULL)
            {
                int value = atoi(tok);
                // printf("    number = %d\n", value);

                for (; counter < value; counter++)
                {
                    this_button <<= 1;
                }
                this_button |= 0b0000000000000001;
                counter += 1;
                this_button <<= 1;

                tok = strtok(NULL, ",");
            }
            for (; counter < 16 - 1; counter++)
            {
                this_button <<= 1;
            }
            // print_binary16(this_button);
            buttons[button_count] = this_button;

            s += mparen[0].rm_eo;
            button_count++;
        }

        struct Machine machine;

        machine.joltage_count = joltage_count;
        machine.state = state;
        machine.button_count = button_count;
        memcpy(machine.buttons, buttons, sizeof(machine.buttons));
        memcpy(machine.joltages, joltages, sizeof(machine.joltages));

        machines[line_count] = machine;

        // printf("\n");
        line_count += 1;
    }

    for (int i = 0; i < line_count; i++)
    {
        // printf("%d\n", machines[i].state);
    }

    // Go over each line
    for (int machine = 0; machine < line_count; machine++)
    {
        struct Machine this_machine = machines[machine];

        // 0
        // 1
        // 2
        // 0 1
        // 0 2
        // 0 3
        // 1 1

        // Iterate over longer and longer perms
        // for (int perms_length = 1;; perms_length++)
        // {
        int to_test[10] = {0};
        while (1)
        {
            int curr_max_digits = 0;

            // Add one
            to_test[0] += 1;

            // Go through each one, and increment if it's too high
            for (int increase = 0; increase < 10; increase++)
            {
                if (to_test[increase] >= this_machine.button_count)
                {
                    to_test[increase] = 0;
                    to_test[increase + 1] += 1;
                }

                if (to_test[increase] > 0)
                {
                    curr_max_digits = increase;
                }
            }

            // Do the check
            u_int16_t new_state = this_machine.state;
            // Apply each of the buttons
            for (int x = 0; x < curr_max_digits; x++)
            {
                new_state ^= this_machine.buttons[to_test[x]];
            }

            if (new_state == 0)
            {
                total += curr_max_digits;
                break;
            }

            // Debug what we're at
            // for (int increase = 0; increase < 10; increase++)
            // {
            //     printf("%d ", to_test[increase]);
            // }
            // printf("\n");
        }
    }

    free(line);
    fclose(fp);

    regfree(&re_pat);
    regfree(&re_parens);
    regfree(&re_curly);

    printf("Part 2 %d", total);

    return 0;
}
