filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().split("\n\n")
    template = input[0]
    rules = input[1].split("\n")
f.close()

insert = {}

for rule in rules:
    insert[rule[:2]] = rule[-1]

polymer = [c for c in template]
steps = 10
prev_step_result = polymer
for step in range(steps):
    result = []
    first_letter = prev_step_result[0]
    result.append(first_letter)
    for index in range(len(prev_step_result)-1):
        second_letter = prev_step_result[index+1]
        insert_letter = insert[''.join([first_letter, second_letter])]
        result.append(insert_letter)
        result.append(second_letter)
        first_letter = second_letter
    prev_step_result = result
    print(f"Finished Step: {step}")


most_frequent = max(set(result), key=result.count)
least_frequent = min(set(result), key=result.count)
print(most_frequent, least_frequent)
print(result.count(most_frequent)-result.count(least_frequent))