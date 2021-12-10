filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().splitlines()
f.close()

# Inputs is input[row][column]
low_points = 0
for row_index, row in enumerate(input):
    for column_index, value in enumerate(row):
        up, down, left, right = 9, 9, 9, 9
        value = int(value)
        if row_index >= 1:
            up = int(input[row_index-1][column_index])
        if row_index <= len(input)-2:
            down = int(input[row_index+1][column_index])
        if column_index >= 1:
            left = int(input[row_index][column_index-1])
        if column_index <= len(row)-2:
            right = int(input[row_index][column_index+1])
        if value < up and value < down and value < left and value < right:
            low_points += value+1

print(low_points)