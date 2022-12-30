with open("data23.txt") as f:
    data = f.read().strip()


def points(elves):

    minx = min([x[0] for x in elves.values()])
    maxx = max([x[0] for x in elves.values()])
    miny = min([x[1] for x in elves.values()])
    maxy = max([x[1] for x in elves.values()])

    width = maxx - minx + 1
    height = maxy - miny + 1

    return width * height - len(elves)

def solve(data):
    # print(data)

    elves = dict()
    i = 0
    for row, line in enumerate(data.splitlines()):
        for col, char in enumerate(line):
            if char == "#":
                elves[i] = (row, col)
                i += 1


    neighbors = [-1,0,1]
    neighbors = [(x,y) for x in neighbors for y in neighbors if not (x == 0 and y == 0)]
    dirs = [(-1,0), (1,0), (0,-1), (0,1)]

    def check_n(elf):
        for n in neighbors:
            if (elf[0] + n[0], elf[1] + n[1]) in elves.values():
                return True
        return False

    def round(elves, i):
        pot_moves = dict()
        for key, value in elves.items():
            if not check_n(value):
                # print("skipped, elf alone")
                continue
            for j in range(i, i+4):
                dir = dirs[j%4]
                new_pos = (value[0] + dir[0], value[1] + dir[1])
                if dir[0] == 0:
                    mods = [(dir[0] + x,dir[1]) for x in [-1,0,1]]
                else:
                    mods = [(dir[0],dir[1] + x) for x in [-1,0,1]]
                # print(f"proposed move: {new_pos}")
                new_poss = [(value[0] + x[0], value[1] + x[1]) for x in mods]
                check = all([x not in elves.values() for x in new_poss])
                if check:
                    if new_pos not in pot_moves:
                        pot_moves[new_pos] = [key]
                    else:
                        pot_moves[new_pos] += [key]
                    break

        for x in pot_moves:
            if len(pot_moves[x]) == 1:
                elves[pot_moves[x][0]] = x
            else:
                # print("do nothing")
                continue
            # print(f"moves {x}:: elfs {pot_moves[x]}")
        return elves

    target = 10
    print(f"start:")
    vis(elves)
    for i in range(target):
        elves = round(elves, i)
        print(f"round {i}")
        # for key, value in elves.items():
            # print(f"Elf {key} is at {value}")

        # vis(elves)

    print(f"Points: {points(elves)}")

def vis(elves):
    minx = min([x[0] for x in elves.values()])
    maxx = max([x[0] for x in elves.values()])
    miny = min([x[1] for x in elves.values()])
    maxy = max([x[1] for x in elves.values()])

    width = maxy +1
    height = maxx + 1

    map = [["." for x in range(width)] for y in range(height)]
    for x in range(height):
        for y in range(width):
            if (x,y) in elves.values():
                map[x][y] = "#"
    for x in map:
        print("".join(x))

solve(data)
