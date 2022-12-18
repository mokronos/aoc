with open("data17.txt", "r") as f:
    data = f.read().strip()

def solve(data):
    data = list(data)

    hor = [["..####."]]
    cross = [["...#..."],
             ["..###.."],
             ["...#..."]]
    step = [["....#.."],
            ["....#.."],
            ["..###.."]]
    vert = [["..#...."],
            ["..#...."],
            ["..#...."],
            ["..#...."]]
    rect = [["..##..."],
            ["..##..."]]
    pieces = [hor, cross, step, vert, rect]

    for i in range(len(pieces)):
        for j in range(len(pieces[i])):
            pieces[i][j] = list(pieces[i][j][0])
        pieces[i] = pieces[i][::-1]

    positions = []

    for piece in pieces:
        pos = []
        for i in range(len(piece)):
            for j in range(len(piece[i])):
                if piece[i][j] == "#":
                    pos.append([i,j])
        positions.append(pos)


    map = [[0,i] for i in range(7)]
    amount = 2022
    amount = int(10e+11)
    top_line = 0
    wind_idx = 0
    wind_length = len(data)
    statemem = {}
    topmem = {}
    before = True
    top = 0
    old_top = 0

    i = 0
    glob_i = 0

    while glob_i<amount:

        adj = top_line + 4

        p = positions[i%5]

        start_pos = [[x+adj,y] for x,y in p]
        if i%100==0:
            print(f"i: {i}")
        # print(f"start_pos: {start_pos}")
        # vis(map, start_pos)
        c = True
        while c:

            wind = data[wind_idx%wind_length]
            pos_wind = [[x,y+1] if wind == ">" else [x,y-1] for x,y in start_pos]
            wind_idx += 1
            new_start_pos = start_pos
            if not collision(map, pos_wind):
                new_start_pos = pos_wind


            start_pos = new_start_pos
            drop_pos = [[x-1,y] for x,y in new_start_pos]
            if not collision(map, drop_pos):
                start_pos = drop_pos
            else:
                c = False

            # print(wind)
            # print(wind_idx-1)
            # print((wind_idx-1)%wind_length)
            # print(f"old pos: {start_pos}")
            # print(f"new pos wind: {pos_wind}")
            # print(f"after wind")
            # vis(map,pos_wind)
            # print(f"old pos: {new_start_pos}")
            # print(f"new pos down: {drop_pos}")
            # print(f"after down")
            # vis(map,drop_pos)

        map.extend(start_pos) 
        i += 1
        glob_i += 1

        top_line = max([x[0] for x in map])

        idx, state = get_state(map, i, wind_idx%wind_length)
        # print(f"idx: {idx}")
        # print(f"state: {state}")
        if state in statemem.values() and before:
            before = False
            print(f"found cycle at {i}")
            t = [k for k,v in statemem.items() if v == state][0]
            t_dif = i - t
            new_i = i
            old_top = topmem[t]
            added_top = 0
            top_dif = top_line - old_top
            div = (amount-glob_i)//t_dif
            glob_i += div*t_dif
            added_top = div*top_dif


        statemem[idx] = state
        topmem[idx] = top_line
        
        # print(f"after whole stone")
        # vis(map)
        # print(f"top_line: {top_line}")
        # print("*"*100)
    print(top_line+ added_top)

def collision(map, pos):
    for x in pos:
        if x in map or x[1] < 0 or x[1] > 6:
            return True
    
def get_state(map, total_idx, wind_idx):

    rock_idx = total_idx%5
    depths = get_depths(map)
    state = [rock_idx, wind_idx, depths]
    return total_idx, state

def get_depths(map):
    map.sort(reverse=True)
    depths = {}
    top = map[0][0]
    to_check = list(range(7))
    for pos in map:
        if pos[1] in to_check:
            depths[pos[1]] = top-pos[0]
            to_check.remove(pos[1])
    return depths
def get_top_levels(map, x=30):
    map.sort(reverse=True)
    result = []
    top = map[0][0]
    target = top - x
    for pos in map:
        if pos[0] > target:
            result.append(pos)
            
    return result

# cut bottom of map off if whole row is filled
def cut(map):
    map.sort(reverse=True)
    count = 0
    mem = map[0][0]
    for i, (x,_) in enumerate(map):
        if x == mem:
            count += 1
            if count == 7:
                # print(f"cutting at {i}")
                return map[:i+1]
        else: 
            mem = x
            count = 1
    return map



def vis(map, pot=[]):
    top_line = max([x[0] for x in map + pot])
    drawn = [["." for _ in range(7)]+[str(i)] for i in range(top_line+1)]
    for pos in map:
        drawn[pos[0]][pos[1]] = "#"
    for pos in pot:
        drawn[pos[0]][pos[1]] = "@"
    for x in drawn[::-1]:
        print("".join(x))
    
solve(data)

