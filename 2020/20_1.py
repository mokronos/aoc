x = open("data20.txt").read()

tiles_raw = x.strip().split("\n\n")

tiles = {}

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

eta
corners = [idn for idn,tile in tiles.items() if len(tile["nb"])==2]
res = 1
for corner in corners:
    res *= corner
print(res)
