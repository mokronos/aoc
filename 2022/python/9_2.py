data = open("data9.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    
    pos = [[0,0] for _ in range(10)]
    post_hist = [pos[9]]

    for line in lines:
        dir, count = line.split(" ")
        count = int(count)
        for _ in range(count):
            pos1, pos2 = newpos(pos[0], pos[1], dir)
            pos[0] = pos1
            pos[1] = pos2
            for i in range(1, len(pos)-1):
                _ , pos2 = newpos_middle(pos[i], pos[i+1])
                pos[i+1] = pos2
            post_hist.append(pos[9][:])

    print(f"final position head: {pos[0]}")
    print(f"final position tail: {pos[9]}")
    print(post_hist)
    print(len(post_hist))
    post_hist = set(tuple(x) for x in post_hist)
    print(post_hist)
    print(f"number of unique positions: {len(post_hist)}")

def newpos(posh, post, dir):

    dirs = {"U": (1,1), "D": (1,-1), "L": (0,-1), "R": (0,1)}

    newposh = [x for x in posh]
    change = dirs[dir]
    newposh[change[0]] += change[1]

    distx = newposh[0] - post[0]
    disty = newposh[1] - post[1]

    if distx in [-1,0,1] and disty in [-1,0,1]:
        return newposh, post

    else:
        return newposh, posh

def newpos_middle(posh, post):

    change_dir = {(2,0):(1,0), (-2,0):(-1,0), (0,2):(0,1), (0,-2):(0,-1),
            (2,1):(1,1), (2,-1):(1,-1),
            (-2,1):(-1,1), (-2,-1):(-1,-1),
            (1,2):(1,1), (-1,2):(-1,1),
            (1,-2):(1,-1), (-1,-2):(-1,-1),
            (2,2):(1,1), (-2,-2):(-1,-1),
            (2,-2):(1,-1), (-2,2):(-1,1),
            }

            
    distx = posh[0] - post[0]
    disty = posh[1] - post[1]

    
    if distx in [-1,0,1] and disty in [-1,0,1]:
        return posh, post

    else:
        change = change_dir[(distx, disty)]
        post[0] += change[0]
        post[1] += change[1]
        return posh, post
        

solve(data)

