### Part One

filename = "input.txt"

with open(filename, "r") as f:
    fishes = f.read().split(",")
    fishes = list(map(int, fishes))
f.close()

for day in range(80):
    for index, fish in enumerate(fishes):
        if fish == 0:
            fishes.append(9)
            fishes[index] = 6
        else:
            fishes[index] = fish-1
    print(f"Finished Day {day+1}")

print(len(fishes))
