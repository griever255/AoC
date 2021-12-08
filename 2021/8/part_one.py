filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().splitlines()
f.close()

codes = []
for line in input:
    codes.append(line.split(" | "))

unique_nums = {
    2: 1,
    4: 4,
    3: 7,
    7: 8
}

count = 0
for index, code in enumerate(codes):
    outputs = codes[index][1].split()
    for output in outputs:
        if len(output) in unique_nums:
            count += 1

print(count)