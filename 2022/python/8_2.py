data = open("data8.txt", "r").read()

def solve(data):
    map = data.splitlines()
    map = [[int(char) for char in line] for line in map]
    for line in map:
        print(line)

    max_score = 0
    for i in range(0, len(map)):
        for j in range(0, len(map[0])):
            score = check_score(i, j, map)
            if score > max_score:
                max_score = score

    print(max_score)

def check_score(i, j, map):

    height = map[i][j]
    maxr = 0
    for tree in map[i][j+1:]:
        if tree >= height :
            maxr += 1
            break
        maxr += 1

    maxl = 0
    for tree in map[i][:j][::-1]:
        if tree >= height :
            maxl += 1
            break
        maxl += 1

    maxd = 0
    tmp = [map[k][j] for k in range(i+1, len(map))]
    for tree in tmp:
        if tree >= height :
            maxd += 1
            break
        maxd += 1

    maxu = 0
    tmp = [map[k][j] for k in range(i)]
    for tree in tmp[::-1]:
        if tree >= height :
            maxu += 1
            break
        maxu += 1

    print(f"posi: {i}, posj: {j}")
    print(f"maxr: {maxr}, maxl: {maxl}, maxu: {maxu}, maxd: {maxd}")

    return maxr * maxl * maxu * maxd

def max_2d(map, j, start, end):
    max = 0
    for i in range(start, end):
        if map[i][j] > max:
            max = map[i][j]
    return max

solve(data)
