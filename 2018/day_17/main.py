from collections import defaultdict
import time
from copy import deepcopy

with open('input.txt') as file:
	lines = [line.replace('\n', '') for line in file]

h = len(lines[0])
w = len(lines)

map = [['.' for x in range(h)] for y in range(w)]

for line in lines:

	first = line.split(',')[0].split('=')[1]
	second = 
	print(first)
	exit()
	if line[0] == 'x':

		first = 'x'
		second = 'y'
	else:
		first = 'y'
		second = 'x'

	parts = line.split(",")

for y, row in enuerate(lines):
	for x, char in enumerate(row):
		map[y][x] = char