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
    top_line = 0
    wind_idx = 0
    wind_length = len(data)

    for i in range(amount):

        adj = top_line + 4

        p = positions[i%5]

        start_pos = [[x+adj,y] for x,y in p]
        if i%100==0:
            print(f"i: {i}")
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

        map.extend(start_pos) 
        
        top_line = max([x[0] for x in map])
    print(top_line)

def collision(map, pos):
    for x in pos:
        if x in map or x[1] < 0 or x[1] > 6:
            return True
    
# just for visulization/debugging
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
