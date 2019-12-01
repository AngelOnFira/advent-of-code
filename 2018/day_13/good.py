s = input()
grid = []
carts = []
while s != 'done':
    row = list(s)
    grid.append(row)
    s = input()
    
for i in range(0, len(grid)):
    for j in range(0, len(grid[i])):
        if grid[i][j] in ['>', '<', '^', 'v']:
            carts.append((i,j,grid[i][j],1))
        v = grid[i][j]
        if v == '>':
            grid[i][j] = '-'
        if v == '<':
            grid[i][j] = '-'
        if v == '^':
            grid[i][j] = '|'
        if v == 'v':
            grid[i][j] = '|'
    
def step(grid, carts):
    carts.sort()
    locs = [(i,j) for (i,j,v,t) in carts]
    newcarts = []
    dont = set()
    while len(carts) > 0:
        (i,j,v,t) = carts.pop(0)
        
        locs = set([(i,j) for (i,j,v,t) in carts])
        locs = locs | set([(i,j) for (i,j,v,t) in newcarts])
        
        if (i,j) in dont:
            continue
        if v == '>':
            # inc j
            # print('locs', locs)
            # print('carts', carts)
            # print('new', newcarts)
            if (i,j+1) in locs:
                print("Collision at", i, j+1)
                dont.add((i,j+1))
                continue
                
            if grid[i][j + 1] == '\\':
                newcarts.append((i,j+1,'v',t))
            elif grid[i][j + 1] == '/':
                newcarts.append((i,j+1,'^',t))
            elif grid[i][j + 1] == '+':
                if t == 1:
                    newcarts.append((i,j+1,'^',2))
                elif t == 2:
                    newcarts.append((i,j+1,'>',3))
                elif t == 3:
                    newcarts.append((i,j+1,'v',1))
            else:
                newcarts.append((i,j+1,'>',t))
                
        if v == '<':
            if (i,j-1) in locs:
                print("Collision at", i, j-1)
                dont.add((i,j-1))
                continue
                
            # dec j
            if grid[i][j - 1] == '\\':
                newcarts.append((i,j-1,'^',t))
            elif grid[i][j - 1] == '/':
                newcarts.append((i,j-1,'v',t))
            elif grid[i][j - 1] == '+':
                if t == 1:
                    newcarts.append((i,j-1,'v',2))
                elif t == 2:
                    newcarts.append((i,j-1,'<',3))
                elif t == 3:
                    newcarts.append((i,j-1,'^',1))
            else:
                newcarts.append((i,j-1,'<',t))
                
        if v == '^':
            if (i-1,j) in locs:
                print("Collision at", i-1, j)
                dont.add((i-1,j))
                continue
                
            # dec i
            if grid[i-1][j] == '\\':
                newcarts.append((i-1,j,'<',t))
            elif grid[i-1][j] == '/':
                newcarts.append((i-1,j,'>',t))
            elif grid[i-1][j] == '+':
                if t == 1:
                    newcarts.append((i-1,j,'<',2))
                elif t == 2:
                    newcarts.append((i-1,j,'^',3))
                elif t == 3:
                    newcarts.append((i-1,j,'>',1))
            else:
                newcarts.append((i-1,j,'^',t))
                
        if v == 'v':
            if (i+1,j) in locs:
                print("Collision at", i, j+1)
                dont.add((i+1,j))
                continue
                
            # dec j
            if grid[i+1][j] == '\\':
                newcarts.append((i+1,j,'>',t))
            elif grid[i+1][j] == '/':
                newcarts.append((i+1,j,'<',t))
            elif grid[i+1][j] == '+':
                if t == 1:
                    newcarts.append((i+1,j,'>',2))
                elif t == 2:
                    newcarts.append((i+1,j,'v',3))
                elif t == 3:
                    newcarts.append((i+1,j,'<',1))
            else:
                newcarts.append((i+1,j,'v',t))
    nc = []
    for (i,j,v,t) in newcarts:
        if (i,j) not in dont:
            nc.append((i,j,v,t))
    return nc
            
    # step
tick = 0
while carts != False:
    # print(tick, len(carts))
    c = step(grid, carts)
    if c == False:
        print(carts)
    if len(c) <= 1:
        # print(c)
        cart = c.pop(0)
        print("Last Cart: {},{}".format(cart[1], cart[0]))
        break
    # print(len(c))
    carts = c
    tick += 1