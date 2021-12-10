filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().splitlines()
f.close()

single_chunks = ["()","[]","{}","<>"]

points = {
    ")": 3,
    "]": 57,
    "}": 1197,
    ">": 25137
}

corrupt = [")", "]", "}", ">"]

total_points = 0
for line in input:
    while any([single_chunk in line for single_chunk in single_chunks]):
        for chunk in single_chunks:
            line = line.replace(chunk, "")
    for char in line:
        if char in corrupt:
            total_points += points[char]
            break
print(total_points)