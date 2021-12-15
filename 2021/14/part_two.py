from os import link


filename = "input.txt"

with open(filename, "r") as f:
    input = f.read().split("\n\n")
    template = input[0]
    rules = input[1].split("\n")
f.close()

insert = {}
count_dict = {}

for rule in rules:
    insert[rule[:2]] = rule[-1]
    count_dict[rule[:2]] = 0

## Step Zero
polymer = [c for c in template]
for i in range(len(polymer)-1):
    count_dict[''.join(polymer[i:i+2])] += 1

## Steps 1-40
steps = 40
for step in range(steps):
    new_count = dict.fromkeys(count_dict, 0)
    for key in count_dict:
        value1 = key[0]+insert[key]
        value2 = insert[key]+key[1]
        new_count[value1] += count_dict[key]
        new_count[value2] += count_dict[key] 
    count_dict = new_count

letter_totals = {}
for key in count_dict:
    if key[0] not in letter_totals:
        letter_totals[key[0]] = count_dict[key]
    else:
        letter_totals[key[0]] += count_dict[key]
    if key[1] not in letter_totals:
        letter_totals[key[1]] = count_dict[key]
    else:
        letter_totals[key[1]] += count_dict[key]

print(round((max(letter_totals.values())-min(letter_totals.values()))/2))