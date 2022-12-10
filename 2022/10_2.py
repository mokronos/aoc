data = open("data10.txt", "r").read()

def solve(data):
    lines = data.splitlines()

    x = [1]
    mem = 0
    for line in lines:
        if line.startswith("n"):
            x.append(x[-1]+mem) 
            mem = 0
        else:
            x.append(x[-1]+mem) 
            mem = 0
            x.append(x[-1]) 
            mem = int(line.split()[-1])
    x.append(x[-1]+mem)

    result = 0
    cycles = [20,60,100,140,180,220]
    x = x[1:-1]

    crt = [["." for _ in range(40)] for _ in range(6)]

    for idx, value in enumerate(x):
        row = idx//40
        pos = idx % 40
        if pos in [value-1, value, value+1]:
            crt[row][pos] = "#"

    crt = ["".join(x) for x in crt]
    for line in crt:
        print(line)
solve(data)
