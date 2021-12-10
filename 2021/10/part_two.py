filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().splitlines()
f.close()

single_chunks = ["()","[]","{}","<>"]

points = {
    "(": 1,
    "[": 2,
    "{": 3,
    "<": 4
}

corrupt = [")", "]", "}", ">"]

scores = []
for line in input:
    score = 0
    incomplete = True
    while any([single_chunk in line for single_chunk in single_chunks]):
        for chunk in single_chunks:
            line = line.replace(chunk, "")
    for char in line:
        if char in corrupt:
            incomplete = False
            break
    if incomplete:
        line = line[::-1]
        for char in line:
            score *= 5
            score += points[char]
        scores.append(score)

middle = len(scores)//2
print(sorted(scores)[middle])
