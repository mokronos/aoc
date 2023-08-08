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
    for cycle in cycles:
        result += x[cycle] * cycle

    print(result)
solve(data)
