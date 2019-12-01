from collections import defaultdict
import time

with open('input.txt') as file:
    lines = [line.replace('\n', '') for line in file]

h = len(lines[0])
w = len(lines)

#print(w, h)

Matrix = [[0 for x in range(h)] for y in range(w)]

carts = []

for y, line in enumerate(lines):
    for x, char in enumerate(lines[y]):
        if char == '>':
            carts.append([x, y, 'r', 'l'])
            Matrix[y][x] = '-'
        elif char == '<':
            carts.append([x, y, 'l', 'l'])
            Matrix[y][x] = '-'
        elif char == 'v':
            carts.append([x, y, 'd', 'l'])
            Matrix[y][x] = '|'
        elif char == '^':
            carts.append([x, y, 'u', 'l'])
            Matrix[y][x] = '|'
        else:
            #print(x, y)
            Matrix[y][x] = char

done = False
while(True):
    delList = []
    carts = sorted(carts, key=lambda x: (x[1], x[0]))

    out = [[' ' for x in range(h)] for y in range(w)]
    for cart in carts:
        out[cart[0]][cart[1]] = '0'

    with open('out.txt', 'w') as f:
        lin = ''
        for item in out:
            for char in item:
                lin += str(char)
                f.write("%s\n" % lin)

    for i, cart in enumerate(carts):
        for j, other_cart in enumerate(carts):
            if cart[0] == other_cart[0] and cart[1] == other_cart[1] and i != j:
                if carts[i] not in delList: delList.append(carts[i])
                if carts[j] not in delList: delList.append(carts[j])

    if len(delList) > 0:
        print(len(carts))
        print(delList)
        for item in delList:
            carts.remove(item)

    if done:
        print(carts)
        exit()

    if len(carts) == 1:
        done=True
        print(carts)
        exit()


    #print(carts)

    for cart in carts:
        tile_on = Matrix[cart[1]][cart[0]]

        if tile_on == '-':
            if cart[2] == 'r': cart[0] += 1
            elif cart[2] == 'l': cart[0] -= 1
        elif tile_on == '|':
            if cart[2] == 'd': cart[1] += 1
            elif cart[2] == 'u': cart[1] -= 1
        elif tile_on == '/':
            if cart[2] == 'r':
                cart[2] = 'u'
                cart[1] -= 1
            elif cart[2] == 'l':
                cart[2] = 'd'
                cart[1] += 1
            elif cart[2] == 'd':
                cart[2] = 'l'
                cart[0] -= 1
            elif cart[2] == 'u':
                cart[2] = 'r'
                cart[0] += 1
        elif tile_on == '\\':
            if cart[2] == 'r':
                cart[2] = 'd'
                cart[1] += 1
            elif cart[2] == 'l':
                cart[2] = 'u'
                cart[1] -= 1
            elif cart[2] == 'd':
                cart[2] = 'r'
                cart[0] += 1
            elif cart[2] == 'u':
                cart[2] = 'l'
                cart[0] -= 1
        elif tile_on == '+':
            if cart[3] == 'l':
                if cart[2] == 'l':
                    cart[2] = 'd'
                    cart[1] += 1
                elif cart[2] == 'd':
                    cart[2] = 'r'
                    cart[0] += 1
                elif cart[2] == 'r':
                    cart[2] = 'u'
                    cart[1] -= 1
                elif cart[2] == 'u':
                    cart[2] = 'l'
                    cart[0] -= 1
                cart[3] = 's'
            elif cart[3] == 's':
                if cart[2] == 'l':
                    cart[0] -= 1
                elif cart[2] == 'd':
                    cart[1] += 1
                elif cart[2] == 'r':
                    cart[0] += 1
                elif cart[2] == 'u':
                    cart[1] -= 1
                cart[3] = 'r'
            elif cart[3] == 'r':
                if cart[2] == 'l':
                    cart[2] = 'u'
                    cart[1] -= 1
                elif cart[2] == 'd':
                    cart[2] = 'l'
                    cart[0] -= 1
                elif cart[2] == 'r':
                    cart[2] = 'd'
                    cart[1] += 1
                elif cart[2] == 'u':
                    cart[2] = 'r'
                    cart[0] += 1
                cart[3] = 'l'
    time.sleep(2)