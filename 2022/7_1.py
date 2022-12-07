data = open("data7.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    _, total, _ = recur(lines, 0, 0, 0)
    print(total)
        
def recur(lines, i, cur, total):

    if i>=len(lines):
        return cur, i


    while i<len(lines):
        # print(f"i: {i}, cur: {cur}, total: {total}")
        # print(lines[i])
        if lines[i].startswith("$ cd .."):
            if cur < 100000:
                total += cur
            i+=1
            return cur, total, i
        elif lines[i].startswith("$ cd"):
            tmp, total, i  = recur(lines, i+1, 0, total)
            cur += tmp

        elif lines[i].startswith("$ ls"):
            i+=1
        elif lines[i].startswith("dir"):
            i+=1
        else:
            size = int(lines[i].split()[0])
            cur += size
            i+=1

    return cur,total, i
solve(data)
