x = open("data13.txt").read()

def solve(x):

    x = x.split("\n")

    target = int(x[0])
    buses = x[1].split(",")
    while "x" in buses:
        buses.remove("x")
    buses = list(map(int, buses))
    buses.sort()
    mem = [] 
    for i in buses:

        count = 0

        while 1:

            count += i

            if count >= target:
                mem.append(count-target)
                break
    print(min(mem) * buses[mem.index(min(mem))])

solve(x)
