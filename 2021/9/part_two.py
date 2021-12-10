filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().splitlines()
f.close()

def get_neighbors(point):
    up, down, left, right = 9, 9, 9, 9
    row_index = point[0]
    column_index = point[1]
    if row_index >= 1:
        up = int(input[row_index-1][column_index])
    if row_index <= len(input)-2:
        down = int(input[row_index+1][column_index])
    if column_index >= 1:
        left = int(input[row_index][column_index-1])
    if column_index <= len(row)-2:
        right = int(input[row_index][column_index+1])
    return [up, down, left, right]

def get_neighbor_points(point):
    row_index = point[0]
    column_index = point[1]
    return [[row_index-1, column_index],[row_index+1, column_index],[row_index, column_index-1],[row_index, column_index+1]]

# Inputs is input[row][column]
low_points = []
for row_index, row in enumerate(input):
    for column_index, value in enumerate(row):
        point = [row_index, column_index]
        neighbors = get_neighbors(point)
        value = int(value)
        if all([value < neighbor_value for neighbor_value in neighbors]):
            low_points.append([row_index, column_index])

# Attempt to do this recursively
def explore_basin(point):
    visited.append(point)
    neighbors = get_neighbors(point)
    neighbor_points = get_neighbor_points(point)
    for index, neighbor in enumerate(neighbors):
        if neighbor < 9 and neighbor_points[index] not in visited:
            explore_basin(neighbor_points[index])

# Cycle through all low-points and keep track of where we visit
basin_size = []
for low_point in low_points:
    visited = []
    explore_basin(low_point)
    basin_size.append(len(visited))

# Sort and multiply
basin_size = sorted(basin_size, key=None, reverse=True)
answer = basin_size[0]*basin_size[1]*basin_size[2]
print(answer)




