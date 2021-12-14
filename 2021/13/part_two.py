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
page1 = np.zeros([max_y+1, max_x+1])

for coordinate in coordinates:
    page1[coordinate[1], coordinate[0]] += 1

instructions = []
for fold in folds:
    axis = fold[fold.index("=")-1]
    fold_line = int(fold[fold.index("=")+1:])
    instructions = instructions + [[axis, fold_line]]

for instruction in instructions:
    if instruction[0] == "x":
        pad_length = instruction[1]-len(page1[:,instruction[1]+1:][0])
        mirror = page1[:,instruction[1]+1:]
        mirror = np.fliplr(mirror)
        mirror = np.pad(mirror, [(0, 0), (pad_length, 0)], mode='constant', constant_values=0)
        page1 = page1[:,:instruction[1]]
        print(instruction[0], instruction[1], page1.shape, mirror.shape)
        page1 += mirror
    if instruction[0] == "y":
        pad_length = instruction[1]-len(page1[instruction[1]+1:,:])
        mirror = page1[instruction[1]+1:,:]
        mirror = np.flipud(mirror)
        mirror = np.pad(mirror, [(pad_length, 0), (0, 0)], mode='constant', constant_values=0)
        page1 = page1[:instruction[1],:]
        print(instruction[0], instruction[1], page1.shape, mirror.shape)
        page1 += mirror

with open('output.txt', 'w') as f:
    for row in page1:
        f.write("\n")
        for value in row:
            if not value == 0:
                f.write("#")
            else:
                f.write(".")
f.close()

