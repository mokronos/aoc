import re
with open('data22.txt') as f:
    data = f.read()

def vis(map):
    map = map.copy()
    # add numbers to map for rows and columns
    # add them on bottom and right as well
    for idx, line in enumerate(map):
        map[idx] = [str(idx)] + line + [str(idx)]

    map = [[" "] + [str(x) for x in range(len(map[0])-2)]] + map + [[" "] + [str(x) for x in range(len(map[0])-2)]]

    # make the numbers visually align with the columns when printed
    for idx, line in enumerate(map):
        map[idx] = [x.rjust(3) for x in line]
    # print the map
    for line in map:
        print(" ".join(line))


def solve(data):

    data = data.splitlines()

    map = data[:-2]
    dir = data[-1]

    height = len(map)
    width = max([len(x) for x in map])
    for idx, line in enumerate(map):
        map[idx] = [x if x in [".", "#"] else " " for x in line]
        # map[idx] = ["." if x == "#" else x for x in map[idx]]
        map[idx].extend([" "]*(width-len(line)))
        
    edge = dict() 
    r = list(range(50))

    # 0: left, 1:down, 2:right, 3:up
    # left close
    for ad in r:
        new_pos = (50+ad, 49)
        old_pos = (50+ad, 50)
        pos = (*old_pos, *new_pos)
        di = 1
        edge[pos] = [(100, 0+ad), di]

    for ad in r:
        new_pos = (99, 0+ad)
        old_pos = (100, 0+ad)
        pos = (*old_pos, *new_pos)
        di = 2
        edge[pos] = [(50+ad, 50), di]

    # right close
    for ad in r:
        new_pos = (50+ad, 100)
        old_pos = (50+ad, 99)
        pos = (*old_pos, *new_pos)
        di = 3
        edge[pos] = [(49, 100+ad), di]

    for ad in r:
        new_pos = (50, 100+ad)
        old_pos = (49, 100+ad)
        pos = (*old_pos, *new_pos)
        di = 0
        edge[pos] = [(50+ad, 99), di]

    # top middle
    for ad in r:
        new_pos = (-1, 50+ad)
        old_pos = (0, 50+ad)
        pos = (*old_pos, *new_pos)
        di = 2
        edge[pos] = [(150+ad, 0), di]

    for ad in r:
        new_pos = (150+ad, -1)
        old_pos = (150+ad, 0)
        pos = (*old_pos, *new_pos)
        di = 1
        edge[pos] = [(0, 50+ad), di]

    # bottom middle
    for ad in r:
        new_pos = (150, 50+ad)
        old_pos = (149, 50+ad)
        pos = (*old_pos, *new_pos)
        di = 0
        edge[pos] = [(150+ad, 49), di]

    for ad in r:
        new_pos = (150+ad, 50)
        old_pos = (150+ad, 49)
        pos = (*old_pos, *new_pos)
        di = 3
        edge[pos] = [(149, 50+ad), di]

    # top right
    for ad in r:
        new_pos = (-1, 100+ad)
        old_pos = (0, 100+ad)
        pos = (*old_pos, *new_pos)
        di = 3
        edge[pos] = [(199, 0+ad), di]

    for ad in r:
        new_pos = (200, 0+ad)
        old_pos = (199, 0+ad)
        pos = (*old_pos, *new_pos)
        di = 1
        edge[pos] = [(0, 100+ad), di]

    # top left
    for ad in r:
        new_pos = (49-ad, 49)
        old_pos = (49-ad, 50)
        pos = (*old_pos, *new_pos)
        di = 2
        edge[pos] = [(100+ad, 0), di]

    for ad in r:
        new_pos = (100+ad, -1)
        old_pos = (100+ad, 0)
        pos = (*old_pos, *new_pos)
        di = 2
        edge[pos] = [(49-ad, 50), di]

    # bottom right
    for ad in r:
        new_pos = (100+ad, 100)
        old_pos = (100+ad, 99)
        pos = (*old_pos, *new_pos)
        di = 0
        edge[pos] = [(49-ad, 149), di]

    for ad in r:
        new_pos = (49-ad, 150)
        old_pos = (49-ad, 149)
        pos = (*old_pos, *new_pos)
        di = 0
        edge[pos] = [(100+ad, 99), di]



    # 0: left, 1:down, 2:right, 3:up
    dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)]
    vis_dirs = ["<", "v", ">", "^"]
    dir_points = {(0, -1): 2 , (1, 0): 1, (0, 1): 0, (-1, 0): 3}
    # turn right: -1
    # turn left: +1
    r = r"(\d+|[RL])"
    dir = re.findall(r, dir)

    def run(map, pos, dir_idx, dir):

        for d in dir:

            if d == "R":
                dir_idx = (dir_idx - 1)%4
            elif d == "L":
                dir_idx = (dir_idx + 1)%4
            else:
                for _ in range(int(d)):
                    # need to define boundaries, at what coords to jump to what other coords over edges
                    new_pos = ((pos[0] + dirs[dir_idx][0]),(pos[1] + dirs[dir_idx][1]))
                    key = (*pos, *new_pos)
                    # print(pos, new_pos)
                    if key in edge:

                        # print(f"jumping from {pos} to {edge[key][0]}")
                        new_pos = edge[key][0]
                        # print(new_pos)
                        if map[new_pos[0]][new_pos[1]] == "#":
                            break
                        dir_idx = edge[key][1]
                    if map[new_pos[0]][new_pos[1]] == "#":
                        break
                    pos = new_pos
        return pos, dir_idx

    # dir = ["1","L","1","L","1","L","1","L"]
    # print("+++++++++++++++++++++")
    # # loop over all positions in map
    # for i in range(len(map)):
    #     for j in range(len(map[0])):
    #         if map[i][j] == ".":
    #             pos = (i, j)
    #             dir_idx = 2
    #             p, idx = run(map, pos, dir_idx, dir)
    #             # print(p, pos)
    #             # print(idx, dir_idx)
    #             if p != pos or idx != dir_idx:
    #                 print("error!!!")
    #                 print(p, pos)
    #                 print(idx, dir_idx)
    # print(pos)
    # print(dirs[dir_idx])

    pos = (0, map[0].index("."))
    dir_idx = 2
    pos, dir_idx = run(map, pos, dir_idx, dir)

    points = 1000*(pos[0]+1) + 4*(pos[1]+1) + dir_points[dirs[dir_idx]]
    print(points)
    # for x in edge:
    #     print(f"{x} -> {edge[x]}")
solve(data)

