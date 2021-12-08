import itertools

filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().splitlines()
f.close()

codes = []
for line in input:
    codes.append(line.split(" | "))

numbers = {
    'abcefg': '0',
    'cf': '1',
    'acdeg': '2',
    'acdfg': '3',
    'bcdf': '4',
    'abdfg': '5',
    'abdefg': '6',
    'acf': '7',
    'abcdefg': '8',
    'abcdfg': '9'
}

all_permutations = list(itertools.permutations('abcdefg'))
all_permutations = [''.join(permutation) for permutation in all_permutations]

def findMap(inputs):
    for permutation in all_permutations:
        mutated_input = []
        for input in inputs:
            translation = input.maketrans('abcdefg', permutation)
            new_input = input.translate(translation)
            new_input = ''.join(sorted(new_input))
            mutated_input.append(new_input)
        if sorted(mutated_input) == sorted(numbers.keys()):
            return permutation

solved_outputs = []
for index, code in enumerate(codes):
    inputs = codes[index][0].split()
    map = findMap(inputs)
    outputs = codes[index][1].split()
    solved_output = []
    for output in outputs:
        translation = output.maketrans('abcdefg', map)
        new_output = output.translate(translation)
        new_output = ''.join(sorted(new_output))
        solved_output.append(numbers[new_output])
    solved_outputs.append("".join(solved_output))

sum = sum([int(output) for output in solved_outputs])
print(sum)

