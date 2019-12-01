from collections import defaultdict
import time
from copy import deepcopy

with open('input.txt') as file:
	lines = [line.replace('\n', '') for line in file]

h = len(lines[0])
w = len(lines)

goblins = []
elves = []

dmg = 25

map = [[' ' for x in range(h)] for y in range(w)]


def getTurnOrder(elves, goblins):
	units = elves + goblins
	units = sorted(units, key=lambda unit: (unit.y, unit.x))

	return units

def getCollisionMap():
	collision = defaultdict(str)

	for y, row in enumerate(map):
		for x, col in enumerate(row):
			collision[str(x)+','+str(y)] = col

	for unit in (elves):
		collision[str(unit.x)+','+str(unit.y)] = 'E'

	for unit in (goblins):
		collision[str(unit.x)+','+str(unit.y)] = 'G'

	return collision

def printMap(h, w):
	collision = getCollisionMap()
	y = 0
	while y < h:
	#for y in range(9):
		x = 0
		while x < w:
		#for x in range(9):
			print(collision[str(x)+','+str(y)], end=' ')
			x += 1
		print("")
		y += 1
	return

def inRange(x, y, enemies):
	range = []
	locs = [[x+1, y], [x-1, y], [x, y+1], [x, y-1]]

	for unit in enemies:
		if [unit.x, unit.y] in locs:
			range.append(unit)

	if len(range) > 0:
		range = sorted(range, key=lambda unit: (unit.health, unit.y, unit.x))
		return range
	return False

def findReachable(enemies, x, y):
	collision = getCollisionMap()

	possible = []
	for enemy in enemies:
		if collision[str(enemy.x - 1)+','+str(enemy.y)] == '.':
			possible.append([enemy.x - 1, enemy.y])
		if collision[str(enemy.x + 1)+','+str(enemy.y)] == '.':
			possible.append([enemy.x + 1, enemy.y])
		if collision[str(enemy.x)+','+str(enemy.y + 1)] == '.':
			possible.append([enemy.x, enemy.y + 1])
		if collision[str(enemy.x)+','+str(enemy.y - 1)] == '.':
			possible.append([enemy.x, enemy.y - 1])

	#print(possible)

	explored = [[x, y]]
	stack = [[x, y, 0, []]]
	dist = defaultdict(list)
	dist[str(x)+','+str(y)] = [0, [0,0]]

	while len(stack) != 0:
		this = stack.pop(0)
		#print(this)

		# Up
		if collision[str(this[0])+','+str(this[1] - 1)] == '.' and [this[0], this[1] - 1] not in explored:
			path = deepcopy(this[3])
			path.append([0,-1])
			stack.append([this[0], this[1] - 1, this[2] + 1, path])
			explored.append([this[0], this[1] - 1])
			dist[str(this[0])+','+str(this[1] - 1)] = [this[2] + 1, path]
		# Left
		if collision[str(this[0] - 1)+','+str(this[1])] == '.' and [this[0] - 1, this[1]] not in explored:
			path = deepcopy(this[3])
			path.append([-1,0])
			stack.append([this[0] - 1, this[1], this[2] + 1, path])
			explored.append([this[0] - 1, this[1]])
			dist[str(this[0] - 1)+','+str(this[1])] = [this[2] + 1, path]
		# Right
		if collision[str(this[0] + 1)+','+str(this[1])] == '.' and [this[0] + 1, this[1]] not in explored:
			path = deepcopy(this[3])
			path.append([1,0])
			stack.append([this[0] + 1, this[1], this[2] + 1, path])
			explored.append([this[0] + 1, this[1]])
			dist[str(this[0] + 1)+','+str(this[1])] = [this[2] + 1, path]
		# Down
		if collision[str(this[0])+','+str(this[1] + 1)] == '.' and [this[0], this[1] + 1] not in explored:
			path = deepcopy(this[3])
			path.append([0,1])
			stack.append([this[0], this[1] + 1, this[2] + 1, path])
			explored.append([this[0], this[1] + 1])
			dist[str(this[0])+','+str(this[1] + 1)] = [this[2] + 1, path]

	intersect = [value for value in explored if value in possible]
	for item in intersect:
		item.append(dist[str(item[0])+','+str(item[1])])

	out = sorted(intersect, key=lambda place: (place[2][0], place[1], place[0]))

	if len(out) > 0:
		return out[0]
	else:
		return False

class Unit:
	def __init__(self, x=0, y=0, enemies=None, type=None):
		self.health = 200
		self.type = type
		self.x = x
		self.y = y
		self.enemies = enemies

for y, line in enumerate(lines):
	for x, char in enumerate(lines[y]):

		if char in ['#', '.']:
			map[y][x] = char
		else:
			map[y][x] = '.'

			unit = Unit(x,y)
			unit.type = char
			if char == 'G':
				unit.enemies = elves
				goblins.append(unit)
			else:
				unit.enemies = goblins
				elves.append(unit)

printMap(h, w)

turns = 0
while not 0 in [len(elves), len(goblins)]:
	turns += 1
	ordUnits = getTurnOrder(elves, goblins)
	while len(ordUnits) > 0:
		unit = ordUnits.pop(0)
		dest = None
		if not inRange(unit.x, unit.y, unit.enemies):
			dest = findReachable(unit.enemies, unit.x, unit.y)
		if dest:
			unit.x += dest[2][1][0][0]
			unit.y += dest[2][1][0][1]
		range = inRange(unit.x, unit.y, unit.enemies)
		if range:
			enemy = range[0]
			if unit.type == 'E':
				enemy.health -= dmg
			else:
				enemy.health -= 3

			if enemy.health <= 0:
				if unit.enemies == elves:
					print("ded")
					exit()
				if enemy in ordUnits: ordUnits.remove(enemy)
				unit.enemies.remove(enemy)

		if 0 in [len(elves), len(goblins)]:
			turns -= 1
			break

	print("")
	printMap(h, w)
	for boio in sorted((goblins+elves), key=lambda boi: (boi.y, boi.x)): print(boio.health, end=" ")
	#input("")
	time.sleep(0.1)


health = 0
for unit in (elves + goblins):
	health += unit.health


print("Final: " + str(health * turns), str(health), str(turns))