with open("data18.txt", "r") as f:
    data = f.read().strip()

def solve(data):

    cubes = [[int(x) for x in row.split(",")] for row in data.splitlines()]

    total = 6*len(cubes)
    touching = 0
    for cube1 in cubes:
        for cube2 in cubes:
            if check(cube1, cube2):
                touching += 1
    
    print(f"surface area: {total-touching}")

def check(cube1, cube2):
    difs = []
    for x,y in zip(cube1, cube2):
        difs.append(abs(x-y))
    if sum(difs) == 1:
        return True
    else:
        return False
solve(data)

