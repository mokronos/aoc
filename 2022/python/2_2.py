data = open("data2.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    
    total = 0
    for line in lines:
        total += rps(line)

    print(total)



def rps(line):

    a, b = line.split(" ")

    rules = {"A": {"X": 3, "Y": 4, "Z": 8},
             "B": {"X": 1, "Y": 5, "Z": 9},
             "C": {"X": 2, "Y": 6, "Z": 7}}

    points = rules[a][b]
    return points

solve(data)
