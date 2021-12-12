import numpy as np

### Part One
filename = "input.txt"

with open(filename, 'r') as f:
    input = f.read().splitlines()
f.close()

array = np.asarray([[int(c) for c in line] for line in input])

# Recursive Flash function
def flash(array, row_index, column_index):
    flashed_points.append([row_index, column_index])
    for r_splash in range(-1, 2):
        for c_splash in range(-1, 2):
            if row_index+r_splash < 0 or column_index+c_splash < 0 \
                or row_index+r_splash > len(array)-1 or column_index+c_splash > len(row)-1:
                pass
            elif [row_index+r_splash, column_index+c_splash] not in flashed_points:
                array[row_index+r_splash][column_index+c_splash] += 1
                if array[row_index+r_splash][column_index+c_splash] > 9:
                    flash(array, row_index+r_splash, column_index+c_splash)


total_flashes = 0
Total_Steps = 100
for steps in range(Total_Steps):
    array += 1
    flashed_points = []
    flashed_points_prev = ["Some Value that isn't None"]
    while not len(flashed_points) == len(flashed_points_prev):
        flashed_points_prev = flashed_points
        for row_index, row in enumerate(array):
            for column_index, value in enumerate(row):
                if value > 9 and [row_index, column_index] not in flashed_points:
                    flash(array, row_index, column_index)
    for point in flashed_points:
        array[point[0]][point[1]] = 0
    total_flashes += len(flashed_points)

print(total_flashes)