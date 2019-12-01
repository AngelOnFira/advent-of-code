from collections import defaultdict

with open('input.txt') as file:
    lines = [line.strip() for line in file]

live_dict = defaultdict(str)

init = lines[0].split(" ")[2]
for i, char in enumerate(init):
    live_dict[i] = char

live_min = 0
live_max = len(init)

rules = []
for i in range(2, len(lines)):
    this_line = lines[i]
    this_rule = this_line.split(" ")[0]
    this_result = this_line.split(" ")[2]

    rules.append([this_rule, this_result])
    
last_tot = 0
last_diff = 0
for i in range(1000):
    last = ''
    for r in range(live_min, live_max):
        last += live_dict[r]

    for pot in range(live_min - 4, live_max + 4):
        state = ''

        for near_pot in range(pot - 2, pot + 3):
            pos = near_pot - live_min
            if pos >= 0 and pos < len(last):
                state += last[pos]
            else:
                state += '.'

        found = False
        for rule in rules:
            if state == rule[0]:
                found = True
                live_dict[pot] = rule[1]
        if not found:
            live_dict[pot] = '.'

    for key in live_dict.keys():
        if key < live_min and live_dict[key] == '#':
            live_min = key

        if key + 1 > live_max and live_dict[key] == '#':
            live_max = key + 1

    #if i > 990:
    #print(last)
    total = 0
    for a, char in enumerate(last):
        if char == '#':
            add = a + live_min
            total += add

    this_diff = total - last_tot
    #print (this_diff)

    #if this_diff == last_diff:
    print(i, ((50000000000 - (i)) * this_diff) + total)

    last_tot = total
    last_diff = this_diff

    #print(total)