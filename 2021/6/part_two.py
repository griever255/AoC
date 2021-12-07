### Part Two

filename = "input.txt"

with open(filename, "r") as f:
    fishes = f.read().split(",")
    fishes = list(map(int, fishes))
f.close()

fish_dict = {
    0: 0,
    1: 0,
    2: 0,
    3: 0,
    4: 0,
    5: 0,
    6: 0,
    7: 0,
    8: 0
}

for fish in fishes:
    fish_dict[fish] = fish_dict[fish]+1

prev_respawn = 0
for day in range(256):
    respawn = 0
    print(f"Simulating Day {day+1}")
    for key in fish_dict.keys():
        if key == 0:
            fish_dict[8] = fish_dict[key]
            respawn = fish_dict[0]
            fish_dict[0] = 0
        elif key <= 6:
            fish_dict[key-1] = fish_dict[key]
        elif key == 7:
            fish_dict[key-1] = fish_dict[key]+respawn
            fish_dict[key] = prev_respawn
    prev_respawn = respawn

total_fish = 0
for key in fish_dict.keys():
    total_fish += fish_dict[key]

print(total_fish)