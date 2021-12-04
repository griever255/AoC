### Part Two
# Determine the O2 and CO2 ratings

filename = "input.txt"

# Read the binaries from a file and store it a as a variable
with open(filename, 'r') as f:
    input = f.read().splitlines()
f.close()

NUM_INPUTS = len(input)
NUM_BITS = len(input[0])

O2 = []
CO2 = []

# Finds the most common bit value in the bit position, breaks ties by O2/CO2 rules
def desired_bit(input, bit, O2):
    count = 0
    # Cycle through all 1,000 messages
    for status in range(len(input)):
        count += int(input[status][bit])    # Add the value of that message, bit to the count
        # Determine if there's more 1's or 0's
    if count == len(input)/2:
        if O2:
            return "1"
        else:
            return "0"
    elif count > len(input)/2:
        if O2:
            return "1"
        else:
            return "0"
    else:
        if O2:
            return "0"
        else:
            return "1"

# Keeps only the statuses with the val in position bit
def keep_statuses(input, bit, val):
    new_input = []
    for status in range(len(input)):
        if input[status][bit] == val:
            new_input.append(input[status])
    return new_input

# Cycle through each bit and only keep status with the desired bit in each index, break when there's one status left
for bit in range(NUM_BITS):
    input = keep_statuses(input, bit, desired_bit(input, bit, True))
    if len(input) == 1:
        break
O2 = int("".join(input),2)

# Re-load the list and repeat for CO2
with open(filename, 'r') as f:
    input = f.read().splitlines()
f.close()

# Cycle through each bit and only keep status with the desired bit in each index, break when there's one status left
for bit in range(NUM_BITS):
    input = keep_statuses(input, bit, desired_bit(input, bit, False))
    if len(input) == 1:
        break
CO2 = int("".join(input),2)

# Print the product
print(O2*CO2)