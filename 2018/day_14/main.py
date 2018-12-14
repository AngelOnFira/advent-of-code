num = 505961

max = 2

class Node:
	def __init__(self, dataval=None, num=None):
		self.dataval = dataval
		self.nextval = None
		self.num = num

class SLinkedList:
	def __init__(self):
		self.headval = None

scores = SLinkedList()
scores.headval = Node(3, 0)
next = Node(7, 1)

# Link first Node to second node
scores.headval.nextval = next

# Link second Node to third node
next.nextval = scores.headval

elf1_pos = scores.headval
elf2_pos = next

last = next

six = ''

while(True):

	new_recipe = str(elf1_pos.dataval + elf2_pos.dataval)

	for char in new_recipe:
		new_node = Node(int(char), max)
		last.nextval = new_node
		last = new_node
		new_node.nextval = scores.headval
		max += 1

		six += char
		if len(six) > 6:
			six = six[1:]

		print(six, num)
		if six == str(num):
			print(max)
			exit()

	for i in range(1 + elf1_pos.dataval):
		elf1_pos = elf1_pos.nextval

	for i in range(1 + elf2_pos.dataval):
		elf2_pos = elf2_pos.nextval

out = ''

iterator = scores.headval
for i in range(num):
	iterator = iterator.nextval

for i in range(5):
	out += str(iterator.dataval)
	iterator = iterator.nextval

print(out)