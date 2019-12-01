from collections import defaultdict
import time
from copy import deepcopy

with open('input.txt') as file:
	lines = [line.replace('\n', '') for line in file]

h = len(lines[0])
w = len(lines)

map = defaultdict(str)

for y, row in enumerate(lines):
	for x, char in enumerate(row):
		map[str(x)+','+str(y)] = char

def printMap(thisMap):
	for y in range(h):
		for x in range(w):
			print(thisMap[str(x)+','+str(y)], end='')
		print("")

new = deepcopy(map)
period = []
for co in range(2000):
	old = deepcopy(new)

	for y in range(h):
		for x in range(w):
			open, tree, lumb = 0,0,0
			for ys in range(3):
				for xs in range(3):
					if xs != 1 or ys != 1:
						thisChar = old[str(x + xs - 1)+','+(str(y + ys - 1))]
						if thisChar == '.':
							open += 1
						elif thisChar == '|':
							tree += 1
						elif thisChar == '#':
							lumb += 1

			pos = str(x) + ',' + str(y)
			if old[pos] == '.' and tree >= 3:
				new[pos] = '|'

			elif old[pos] == '|' and lumb >= 3:
				new[pos] = '#'

			elif old[pos] == '#' and not (lumb >= 1 and tree >= 1):
				new[pos] = '.'

	out = list(new.values())
	countTrees = out.count('|')
	countLumb = out.count('#')
	score = countTrees * countLumb
	print('Score: ' + str(score),co+1)
	printMap(old)
	input("")

	#if score in period:
	#	print('Same found at ' + str(period.index(score)) + ' from ' + str(co))
	#period.append(score)
	#input("")
print(period)

print(map)