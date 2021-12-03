filename = "input.txt"  # File containing actions separated by a \n


### Part II
# Submarine Class that tracks aim
# Dictionary of actions
# Methods that manipulate the position based on the dictionary value
class Submarine():
    def __init__(self):
        self.horz = 0
        self.aim = 0
        self.depth = 0
        self.actions = {
            "forward": self.go_forward,
            "up": self.aim_up,
            "down": self.aim_down
        }

    def go_forward(self, val):
        self.horz += int(val)
        self.depth += int(val)*self.aim

    def aim_up(self, val):
        self.aim -= int(val)

    def aim_down(self, val):
        self.aim += int(val)

# Read the depths from a file and store it a as a variable
with open(filename, 'r') as f:
    commands = f.read().splitlines()
f.close()

# Create a submarine object
Nautilus = Submarine()

# Each line has a `command, val` format
for command in commands:
    val = command[-1]   # Value is always at the end of the line
    # Loop through all available actions
    for key in Nautilus.actions.keys():
        if key in command:                  # If the action is in the command
            Nautilus.actions[key](val)      # Call the method that manipulates the position

# Print the location and product for the solution to Part I
print(f"Horizontal = {Nautilus.horz}, Depth = {Nautilus.depth}, Product = {Nautilus.horz*Nautilus.depth}")

