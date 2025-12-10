#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <regex.h>

#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))

static const char *const RE_PATTERN = "^\\[([.#]+)\\]";
static const char *const RE_PARENS = "\\(([0-9,]+)\\)";
static const char *const RE_CURLY = "\\{([0-9,]+)\\}";

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
    int stuff[20];
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
        printf("LINE: %s", line);

        uint16_t state = 0;
        uint16_t buttons[15];
        int stuff[20];

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
            // printf("Curly:   \"%.*s\"\n", clen, line + mcurly[1].rm_so);
        }

        printf("Parens:\n");
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
                printf("    number = %d\n", value);

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
            print_binary16(this_button);
            buttons[button_count] = this_button;

            s += mparen[0].rm_eo;
            button_count++;
        }

        struct Machine machine;

        machine.state = state;
        machine.button_count = button_count;
        memcpy(machine.buttons, buttons, sizeof(machine.buttons));
        memcpy(machine.stuff, stuff, sizeof(machine.stuff));

        machines[line_count] = machine;

        printf("\n");
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
            // for (int how_many_perms_to_check = 1; how_many_perms_to_check < perms_length; how_many_perms_to_check++)
            // {
            //     // Go through each index of how many buttons are on this machine
            //     for (int machine_button_index = 0;; machine_button_index++)
            //     {
            //         // If it's 0, then we can break because no buttons have no effect
            //         if (this_machine.buttons[machine_button_index] == 0) {
            //             break;
            //         }

            //         to_test[how_many_perms_to_check] += 1;

            //         // Do the check
            //         u_int16_t new_state = this_machine.state;

            //         // Apply each of the buttons
            //         for (int x = 0; x < how_many_perms_to_check; x++) {
            //             new_state ^= this_machine.buttons[x];
            //         }

            //         if (new_state == 0) {
            //             total += perms_length;
            //             break
            //         }
            //     }
            // }

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

    printf("Part 1 %d", total);

    return 0;
}
