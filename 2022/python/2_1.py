data = open("data2.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    
    total = 0
    for line in lines:
        total += rps(line)

    print(total)



def rps(line):

    a, b = line.split(" ")

    rules = {"A": {"X": 4, "Y": 8, "Z": 3},
             "B": {"X": 1, "Y": 5, "Z": 9},
             "C": {"X": 7, "Y": 2, "Z": 6}}

    points = rules[a][b]
    return points

solve(data)
