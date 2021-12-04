import numpy as np

class Board():
    def __init__(self, numRows, numColumns, values):
        self.numRows = numRows
        self.numColumns = numColumns
        self.values = values
        self.bingo = False
        self.marked_balls = []
        self.unmarked_balls = values.flatten()
    
    def setBingo(self):
        self.bingo = True
    
    def setMarkedBalls(self, marked_balls):
        self.marked_balls = marked_balls
    
    def setUnmarkedBalls(self, unmarked_balls):
        self.unmarked_balls = unmarked_balls
    
    def mark_ball(self, ball):
        self.marked_balls.append(ball)
        self.unmarked_balls = np.delete(self.unmarked_balls, np.where(self.unmarked_balls == ball))

    def check_bingo(self):
        check = False
        for row in self.values:
            for cell in row:
                if cell in self.marked_balls:
                    check = True
                else:
                    check = False
                    break
            if check == True:
                self.setBingo()
                print("BINGO")
                break
        columns = self.values.transpose()
        for column in columns:
            for cell in column:
                if cell in self.marked_balls:
                    check = True
                else:
                    check = False
                    break
            if check == True:
                self.setBingo()
                print("BINGO")
                break
        

# Read the numbers and bingo boards from a file
filename = "input.txt"

Boards = []
with open(filename, 'r') as f:
    # First line is a comma delimited list of balls
    balls = next(f).strip().split(',')
    # Bingo boards are separated by a \n
    boards_input = f.read().strip().split('\n\n')
f.close()

# Create a Board object for each board input
for board in boards_input:
    board = board.splitlines() # Creates a list of rows for each board
    numRows = len(board)
    numColumns = len(board[0].split())
    
    # Create an numRows x numColumns array per board
    values = []
    for row in board:
        values.append(np.array(row.split()))
    values = np.asarray(values)

    # Creates a Board object
    bingo_board = Board(numRows, numColumns, values)
    Boards.append(bingo_board)

# Call balls one at a time, check for bingos
losing_board = None
num_boards = len(Boards)
num_bingos = 0
for ball in balls:
    for board in Boards:
        board.mark_ball(ball)
        if board.bingo == False:
            board.check_bingo()
            if board.bingo == True:
                num_bingos += 1
    if num_bingos == num_boards-1:
        break

# Parse out which board lost
for board in Boards:
    if board.bingo == False:
        losing_board = board

# Run a sim on that losing board until it wins
winning_board = None
losing_board.setMarkedBalls([])
losing_board.setUnmarkedBalls(losing_board.values.flatten())
for ball in balls:
    bingo = False
    losing_board.mark_ball(ball)
    losing_board.check_bingo()
    if losing_board.bingo == True:
        winning_board = board
        bingo = True
    if bingo == True:
        break

# Find the sum of unmarked balls * last ball
sum = 0
for ball in losing_board.unmarked_balls:
    sum += int(ball)
last_ball = int(losing_board.marked_balls[-1])
print(sum*last_ball)
        