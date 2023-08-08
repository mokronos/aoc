with open("data18.txt", "r") as f:
    data = f.read().strip()

def solve(data):

    cubes = {tuple(int(x) for x in row.split(",")) for row in data.splitlines()}

    sides = lambda x,y,z : {(x+1,y,z),(x-1,y,z),(x,y+1,z),(x,y-1,z),(x,y,z+1),(x,y,z-1)}

    print(f"part1: {sum((s not in cubes) for c in cubes for s in sides(*c))}")
    q = [(-1,-1,-1)]
    seen = set()

    while q:
        cur = q.pop()
        for side in sides(*cur)-seen-cubes:
            if all(-1<=c<=25 for c in side):
                q.append(side)
                seen |= {side}

    
    print(f"part2: {sum((s in seen) for c in cubes for s in sides(*c))}")

solve(data)

