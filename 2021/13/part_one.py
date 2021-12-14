import numpy as np
filename = "input.txt"

with open(filename, 'r') as f:
    input = f.read().split("\n\n")
    coordinates = input[0].split("\n")
    coordinates = [coordinate.split(",") for coordinate in coordinates]
    coordinates = np.asarray(coordinates).astype(int)
    folds = input[1].split("\n")
f.close

max_x, max_y = np.amax(coordinates, 0)
max_x += max_x%2+1
max_y += max_y%2+1
page1 = np.zeros([max_y, max_x])

for coordinate in coordinates:
    page1[coordinate[1], coordinate[0]] += 1

instructions = []
for fold in folds:
    axis = fold[fold.index("=")-1]
    fold_line = int(fold[fold.index("=")+1:])
    instructions = instructions + [[axis, fold_line]]

instruction = instructions[0]
if instruction[0] == "x":
    mirror = page1[:,instruction[1]+1:]
    mirror = np.fliplr(mirror)
    page1 = page1[:,:instruction[1]] + mirror
if instruction[0] == "y":
    mirror = page1[instruction[1]+1:,:]
    mirror = np.flipud(mirror)
    page1 = page1[:instruction[1],:] + mirror
print(page1)

count = (page1 >= 1).sum()
print(count)
