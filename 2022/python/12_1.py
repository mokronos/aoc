import string

data = open("data12.txt", "r").read()

def solve(data):
    lines = data.splitlines()

    height = len(lines)
    width = len(lines[0])

    start = (0,0)
    end = (height-1, width-1)
    for i, line in enumerate(lines):
        for j, item in enumerate(line):
            if item == "S":
                start = (i, j)
            if item == "E":
                end = (i, j)

    alph = string.ascii_lowercase
    repl = {x: y for x, y in zip(alph, range(1, 27))}

    for i, line in enumerate(lines):
        lines[i] = list(line)

    for i, line in enumerate(lines):
        for j, item in enumerate(line):
            if item in alph:
                lines[i][j] = repl[item]
    lines[start[0]][start[1]] = 1
    lines[end[0]][end[1]] = 26
    
    # record visited nodes and shortest path from start until now
    mem = [[None for _ in range(width)] for _ in range(height)]
    vis = [[False for _ in range(width)] for _ in range(height)]
    mem[start[0]][start[1]] = 1
    # vis[start[0]][start[1]] = True


    # for x in lines:
    #     print(x)
    to_visit = [start]
    new_to_visit = []
    cycles = (height)*(width)
    while 1:
        for cur_pos in to_visit:
            # print(f"cur_pos: {cur_pos}")
            vis, mem, poss = eva(lines, mem, vis, cur_pos)
            new_to_visit.extend(poss)

            # print("state:")
            # for x in mem:
            #     print(x)
            # print("visited:")
            # for x in vis:
            #     print(x)
        to_visit = new_to_visit
        if len(to_visit) == 0:
            break
        to_visit = list(set(to_visit))
        # print seperation line
        # print("=====================================")
        # print(f"to_visit: {to_visit}")
        # print("=====================================")
        new_to_visit = []

    # print("map:")
    # print("state:")
    # for x in mem:
    #     print(x)
    # print("visited:")
    # for x in vis:
    #     print(x)
    res = mem[end[0]][end[1]]
    # print(f"Start: {start}, End: {end}")
    print(f"min steps: {res-1}")
def eva(lines, mem, vis, pos):
    height = len(lines)
    width = len(lines[0])
    i, j = pos
    poss = []
    if vis[i][j]:
        return vis, mem, poss
    cur = lines[i][j]
    neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    for di, dj in neighbors:
        posi = i + di
        posj = j + dj
        if posi < 0 or posi >= height:
            continue
        if posj < 0 or posj >= width:
            continue
        if vis[posi][posj]:
            continue
        val = lines[posi][posj]
        if val <= cur +1:
            poss.append((posi, posj))
            tmp = mem[i][j] + 1
            if mem[posi][posj] is None or tmp < mem[posi][posj]:
                mem[posi][posj] = tmp
            # print(f"({i}, {j}) -> ({posi}, {posj})")
            # for x in mem:
            #     print(x)
    vis[i][j] = True
    return vis, mem, poss
    

solve(data)
