import re
with open('data22.txt') as f:
    data = f.read()

def solve(data):

    data = data.splitlines()

    map = data[:-2]
    dir = data[-1]

    height = len(map)
    width = max([len(x) for x in map])
    for idx, line in enumerate(map):
        map[idx] = [x if x in [".", "#"] else "x" for x in line]
        map[idx].extend(["x"]*(width-len(line)))
        

    pos = (0, map[0].index("."))
    dir_idx = 2

    dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)]
    dir_points = {(0, -1): 2 , (1, 0): 1, (0, 1): 0, (-1, 0): 3}
    # turn right: -1
    # turn left: +1
    r = r"(\d+|[RL])"
    dir = re.findall(r, dir)
    for d in dir:

        if d == "R":
            dir_idx = (dir_idx - 1)%4
        elif d == "L":
            dir_idx = (dir_idx + 1)%4
        else:
            for i in range(int(d)):
                new_pos = ((pos[0] + dirs[dir_idx][0])%height,(pos[1] + dirs[dir_idx][1])%width)
                if map[new_pos[0]][new_pos[1]] == "#":
                    break
                while not map[new_pos[0]][new_pos[1]] == ".":
                    new_pos = ((new_pos[0] + dirs[dir_idx][0])%height,(new_pos[1] + dirs[dir_idx][1])%width)
                    if map[new_pos[0]][new_pos[1]] == "#":
                        new_pos = pos
                        break

                pos = new_pos

    print(pos)
    print(dirs[dir_idx])

    points = 1000*(pos[0]+1) + 4*(pos[1]+1) + dir_points[dirs[dir_idx]]
    print(points)
solve(data)
