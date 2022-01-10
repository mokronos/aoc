x = open("data20t.txt").read()

tiles_raw = x.strip().split("\n\n")

tiles = {}

tile = ["12","34"]

def rot(tile):
    res = []
    res.append(tile)
    for i in range(3):
        newtile = []
        for j in range(len(tile[0])):
            x = ""
            for k in range(len(tile[0])-1,-1,-1):
                x += tile[k][j]
            newtile.append(x)
        tile = newtile
        res.append(newtile)
        

    for x in res:
        res.append(x[::-1])
    for i in res:
        print(i)
    print("*"*50)
rot(tile)

for tile in tiles_raw:
    lines = tile.strip("\n").split("\n")
    idn = int(lines[0].split(" ")[1].strip(":"))
    square = lines[1:]
    borders = [square[0], square[-1], "".join([x[0] for x in square]),"".join([x[-1] for x in square])]
    borders += [x[::-1] for x in borders]
    tiles[idn] = {"square": square, "borders": borders, "nb": {}}

for i1,tile1 in tiles.items():
    for i2,tile2 in tiles.items():
        if i1 == i2:
            continue

        shared = [x for x in tile1["borders"] if x in tile2["borders"]]
        for s in shared:
            tiles[i2]["nb"][i1] = s
            tiles[i1]["nb"][i2] = s

corners = [idn for idn,tile in tiles.items() if len(tile["nb"])==2]
grid = [[0]]
dim = int(len(tiles)**0.5)
for r in range(dim):
    for c in range(dim):
        if c == r == 0:
            grid[c][r] = tiles[corners[0]]["square"]
            del tiles[corners[0]]
        elif not c == 0:
            bord = "".join([x[-1] for x in grid[r][c-1]])
            for idn, tile in tiles.items():
                if bord in tile["borders"]:
                    grid[r].append(tile["square"])
                    del tiles[idn]
                    break
        elif c == 0:
            bord = grid[r-1][c][-1]
            for idn, tile in tiles.items():
                if bord in tile["borders"]:
                    grid.append(tile["square"])
                    del tiles[idn]
                    break

print(grid)
