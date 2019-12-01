import re
import time

stars = re.compile('position=<([0-9\-\ ]*),([0-9\-\ ]*)> velocity=<([0-9\-\ ]*),([0-9\-\ ]*)>')

w, h = 3000, 3000
x_off, y_off = 1670, 1600
Matrix = [['.' for x in range(w)] for y in range(h)] 

with open('input.txt') as file:
    lines = [line.strip() for line in file]

star_sim = []
for line in lines:
    x_pos, y_pos, x_vel, y_vel = stars.match(line).groups()
    star_sim.append([int(x_pos), int(y_pos), int(x_vel), int(y_vel)])

for i in range(20000):
    for star in star_sim:
        star[0] += star[2]
        star[1] += star[3]
        if star[0] >= x_off - int(w/2) and star[0] < x_off + int(w/2) and star[1] >= y_off - int(h/2) and star[1] < y_off + int(h/2):
            Matrix[star[1] + int(h/2) - y_off][star[0] + int(w/2) - x_off] = '#'

    if i > 10456:
        with open('your_file.txt', 'w') as f:
            f.write("%d\n" % i)
            for row in Matrix:
                this_line = ''
                for char in row:
                    this_line += char
                f.write("%s\n" % this_line)
        print("done")
        time.sleep(5)
    if i > 10420:
        Matrix = [['.' for x in range(w)] for y in range(h)]