with open("data24.txt") as f:
    data = f.read().strip()



def solve(data):
    data = data.splitlines()
    height = len(data)
    width = len(data[0])
    start = (0,1)
    end = (height-1,width-2)

    options = [(0,1), (0,-1), (1,0), (-1,0), (0,0)]
    dirs = {"<" : (0,-1), ">" : (0,1), "^" : (-1,0), "v" : (1,0)}

    blizz = []
    reachable = set()
    for y, line in enumerate(data):
        for x, char in enumerate(line):
            if char in dirs.keys():
                blizz.append(((y,x),dirs[char]))
            if char != "#":
                reachable.add((y,x))
    cand = set()
    cand.add(start)

    def move_blizz(blizz):
        new_blizz = []
        for item in blizz:
            new_y = item[0][0] + item[1][0]
            new_x = item[0][1] + item[1][1]
            if new_x == 0:
                new_x = width-2
            if new_x == width-1:
                new_x = 1
            if new_y == 0:
                new_y = height-2
            if new_y == height-1:
                new_y = 1

            new_blizz.append(((new_y, new_x), item[1]))
        return new_blizz

    def move_cand(cand, blizz):
        new_cand = set()
        blizz_blocked = [x[0] for x in blizz]
        for c in cand:
            for opt in options:
                new_c = (c[0] + opt[0], c[1] + opt[1])
                if new_c not in blizz_blocked and new_c in reachable:
                    new_cand.add(new_c)
        return new_cand

    round = 0
    while end not in cand:
        # move blizzard
        blizz = move_blizz(blizz)
            
        # move candidates
        cand = move_cand(cand, blizz)
        round += 1
        if round%20 == 0:
            print(f"round {round}")
    print(f"round: {round}")

solve(data)
