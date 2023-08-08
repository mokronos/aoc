data = open("data9.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    
    posh = [0,0]
    post = [0,0]
    post_hist = [post]

    for line in lines:
        dir, count = line.split(" ")
        count = int(count)
        for _ in range(count):
            posh, post = newpos(posh, post, dir)
            post_hist.append(post)

    print(f"final position head: {posh}")
    print(f"final position tail: {post}")
    post_hist = set(tuple(x) for x in post_hist)
    print(f"number of unique positions: {len(post_hist)}")

def newpos(posh, post, dir):

    dirs = {"U": (1,1), "D": (1,-1), "L": (0,-1), "R": (0,1)}

    change = dirs[dir]
    newposh = [x for x in posh]
    newposh[change[0]] += change[1]

    distx = newposh[0] - post[0]
    disty = newposh[1] - post[1]

    if distx in [-1,0,1] and disty in [-1,0,1]:
        return newposh, post

    else:
        return newposh, posh


solve(data)

