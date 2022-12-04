data = open("data4.txt", "r").read()

def solve(data):
    data = data.splitlines()

    res = 0
    for line in data:
        if check(line):
            res += 1
    
    print(res)

def check(line):

    a, b = line.split(",")


    astart, aend = list(map(int,a.split("-")))
    bstart, bend = list(map(int,b.split("-")))


    if astart==bstart:
        return True
    elif astart<bstart:
        if aend>=bend:
            return True
    else:
        if bend>=aend:
            return True

    return False


solve(data)
