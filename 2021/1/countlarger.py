# Initialize variables
filename = "depthmeasurements.txt"  # File containing depths separated by a \n
count = 0                           # Initialize deeper depths as zero
previous_depth = None               # Initialize a NoneType previous_depth

# Read the depths from a file and store it a as a variable
with open(filename, 'r') as f:
    depths = f.read().splitlines()
f.close()

### Part One
# Check if each depth is deeper and keep track of count
for depth in depths:
    current_depth = int(depth)
    if previous_depth is not None and current_depth > previous_depth:
        count += 1
    previous_depth = current_depth

# Print the number of depths that are deeper than the previous depth
# print(count)
        
### Part Two
# Check a three-depth sliding window sum
count = 0                       # Initialize deeper windows to zero
window_size = 3                 # Window look-ahead size to three
total_indices = len(depths)     # Maximum Index of Depths (1999)
prev_window_sum = None       # Previous avg window is zero

# Check if sum of indexed depth to the window size is larger than previous window
for index, depth in enumerate(depths):
    current_window_sum = 0
    if index+window_size-1 < total_indices:         # If there are at least window size ahead of indexed depth
        # Look at the current depth and ahead to the size of the window
        for window_index in range(window_size):     
            current_window_sum += int(depths[index+window_index])   # Sum the depths in each window size
    # If the sum is greater than the previous sum, count this depth
    if prev_window_sum is not None and current_window_sum > prev_window_sum:
        count += 1
    prev_window_sum = current_window_sum

# Print the number of depths that are deeper than the previous depth
print(count)
