serial = 9306

x_dim = 300
y_dim = 300

Matrix = [[0 for x in range(x_dim)] for y in range(y_dim)] 

for y in range(1, y_dim + 1):
	for x in range(1, x_dim + 1):
		rack_id = (x + 10)
		power_level = rack_id * y
		power_level += serial
		power_level *= rack_id

		if power_level < 100:
			power_level = 0
		else:
			power_level = int(str(power_level)[-3])

		power_level -= 5

		Matrix[y - 1][x - 1] = power_level
		#print(x, y, power_level)

largest_value = 0
largest_coor = ''
for square_size in range(300):
	for y in range(y_dim - square_size):
		for x in range(x_dim - square_size):
			power_region = 0
			for small_y in range(square_size):
				for small_x in range(square_size):
					power_region += Matrix[y + small_y][x + small_x]

			if power_region > largest_value:
				largest_value = power_region
				largest_coor = str(x + 1) + ',' + str(y + 1)
				print(str(largest_coor) + ',' + str(square_size) + ',' + str(largest_value))

print(largest_coor)