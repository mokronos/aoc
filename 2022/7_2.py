data = open("data7.txt", "r").read()

def solve(data):
    lines = data.splitlines()

    used = 0
    for line in lines:
        if line[0].isdigit():
            used+=int(line.split()[0])

    space = 70000000
    free = space - used
    needed = 30000000
    todelete = needed - free
    print(f"todelete: {todelete}")
    _, total, _, _ = recur(lines, 0, 0, 0, todelete)
        
def recur(lines, i, cur, total, todelete):

    if i>=len(lines):
        return cur, total, i, False

    while i<len(lines):
        # print(f"i: {i}, cur: {cur}, total: {total}")
        # print(lines[i])
        if lines[i].startswith("$ cd .."):
            if cur < 100000:
                total += cur
            i+=1
            return cur, total, i, False
        elif lines[i].startswith("$ cd"):
            tmp, total, i, found  = recur(lines, i+1, 0, total, todelete)
            if found:
                return 0,0,0,True
            if tmp>=todelete:
                print(f"deleted {tmp}")
                return 0,0,0,True
            cur += tmp

        elif lines[i].startswith("$ ls"):
            i+=1
        elif lines[i].startswith("dir"):
            i+=1
        else:
            size = int(lines[i].split()[0])
            cur += size
            i+=1

    return cur,total, i, False
solve(data)
