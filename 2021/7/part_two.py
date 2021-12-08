### Part Two

filename = "input.txt"

with open(filename, "r") as f:
    positions = f.read().split(",")
    positions = list(map(int, positions))
f.close()

min = min(positions)
max = max(positions)

best_fuel = 99999999
for middle in range(min, max):
    fuel = 0
    for position in positions:
        fuel += abs(position-middle)*(abs(position-middle)+1)/2
    if fuel < best_fuel:
        best_fuel = fuel
print(best_fuel)