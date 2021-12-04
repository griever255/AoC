### Part One
# Determine the gamma and episilon
# Most common bit and least common bit

filename = "input.txt"

# Read the binaries from a file and store it a as a variable
with open(filename, 'r') as f:
    input = f.read().splitlines()
f.close()

NUM_INPUTS = len(input)
NUM_BITS = len(input[0])

gamma = []
epsilon = []

# Cycle through each bit index
for bit in range(NUM_BITS):
    count = 0
    # Cycle through all 1,000 messages
    for status in range(NUM_INPUTS):
        count += int(input[status][bit])    # Add the value of that message, bit to the count
    
    # Determine if there's more 1's or 0's
    if count > NUM_INPUTS//2:
        gamma.append("1")
        epsilon.append("0")
    else:
        gamma.append("0")
        epsilon.append("1")

# Convert to decimal
gamma = int("".join(gamma),2)
epsilon = int("".join(epsilon),2)
print(gamma*epsilon)