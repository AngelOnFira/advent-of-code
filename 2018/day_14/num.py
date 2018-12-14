import collections
import re

ser = 9306

d = {}
for i in range(1, 301):
  for j in range(1, 301):
    rack_id = i + 10
    then = (rack_id * j + ser) * rack_id
    powr = ((then // 100) % 10) - 5
    d[(i,j)] = powr


m = -100
mxy = 0
for i in range(1, 301):
  for j in range(1, 301):
    k = d[(i, j)]
    if k > m:
      m = k
      mxy = (i, j)
print (mxy)


# cs[(x, y)] is the cumulative sum of d[(i, j)] for all i <= x and j <= y
cs = {}
for i in range(1, 301):
  for j in range(1, 301):
    cs[(i, j)] = d[(i, j)] + cs.get((i - 1, j), 0) + cs.get((i, j - 1), 0) - cs.get((i - 1, j - 1), 0)
m = -100
mxy = 0
for i in range(1, 301):
  for j in range(1, 301):
    for s in range(1, 301 - max(i, j)):
      # I figured out after submitting that these indices should all be one
      # smaller since the bounds of the square are
      # i <= x < i + s, j <= y < j + s
      k = cs[(i + s, j + s)] + cs[(i, j)] - cs[(i + s, j)] - cs[(i, j + s)]
      if k > m:
        m = k
        mxy = (i, j, s)

print(m, mxy)