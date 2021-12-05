import numpy as np

## Part One
# Read input file and parse list of beginning and end points
filename = "input.txt"

with open(filename, 'r') as f:
    input = f.read().splitlines()
f.close()

# Create an array of [[x0, y0], [x1, y1]] and note the max values
vectors = []
max_x, max_y = 0, 0
for line in input:
    vector = line.split(" -> ")
    beg = vector[0].split(",")
    end = vector[1].split(",")
    x0, x1 = int(beg[0]), int(end[0])
    y0, y1 = int(beg[1]), int(end[1])
    vectors.append([[x0, y0], [x1, y1]])

    if x0 > max_x:
        max_x = x0
    if x1 > max_x:
        max_x = x1
    if y0 > max_y:
        max_y = y0
    if y1 > max_y:
        max_y = y1
vectors = np.asarray(vectors)
field = np.zeros([max_y+1, max_x+1])

# Returns True if the vector [[x0, y0], [x1, y1]] is a line
def isLine(vector):
    x0 = vector[0][0]
    x1 = vector[1][0]
    y0 = vector[0][1]
    y1 = vector[1][1]
    if x0 == x1 or y0 == y1:
        return True
    else:
        return False

def isHorz(vector):
    y0 = vector[0][1]
    y1 = vector[1][1]
    if y0 == y1:
        return True
    else:
        return False

def isVert(vector):
    x0 = vector[0][0]
    x1 = vector[1][0]
    if x0 == x1:
        return True
    else:
        return False

for vector in vectors:
    x0 = vector[0][0]
    x1 = vector[1][0]
    y0 = vector[0][1]
    y1 = vector[1][1]
    if isLine(vector):
        if isVert(vector):
            if y0 < y1:
                for y in range(y1-y0+1):
                    field[y0+y, x0] = field[y0+y, x0]+1
            else:
                for y in range(y0-y1+1):
                    field[y1+y, x0] = field[y1+y, x0]+1
        elif isHorz(vector):
            if x0 < x1:
                for x in range(x1-x0+1):
                    field[y0, x0+x] = field[y0, x0+x]+1
            else:
                for x in range(x0-x1+1):
                    field[y0, x1+x] = field[y0, x1+x]+1

two_or_greater = (field >= 2).sum()
print(field)
print(two_or_greater)