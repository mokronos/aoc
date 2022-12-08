data = open("data8.txt", "r").read()

def solve(data):
    map = data.splitlines()
    map = [[int(char) for char in line] for line in map]
    for line in map:
        print(line)

    visible = 0

    for i in range(1, len(map)-1):
        for j in range(1, len(map[0])-1):
            if check(i, j, map):
                visible += 1
                print(f"visible: {visible}")
    visible += len(map)*2 + len(map[0])*2 - 4
    print(visible)

def check(i, j, map):


    maxr = max(map[i][j+1:])
    maxl = max(map[i][:j])
    maxu = max_2d(map, j, 0, i)
    maxd = max_2d(map, j, i+1, len(map))
    print(f"posi: {i}, posj: {j}")
    print(f"maxr: {maxr}, maxl: {maxl}, maxu: {maxu}, maxd: {maxd}")

    if min(maxr, maxl, maxu, maxd) >= map[i][j]:
        return False
    else:
        return True



def max_2d(map, j, start, end):
    max = 0
    for i in range(start, end):
        if map[i][j] > max:
            max = map[i][j]
    return max

solve(data)
