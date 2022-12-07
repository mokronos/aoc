data = open("data7.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    total, _ = recur(lines, 0, [], 0)
    print(total)
        
def recur(lines, cur, i, total):
    cur = 0
    line = lines[i]
    if line.startswith("$ ls"):
        while not lines[i].startswith("$"):
            if lines[i].startswith("dir"):
                i+=1
            size = int(line.split()[0])
            cur += size
            i+=1
    elif line.startswith("$ cd "):
        tmp, i = recur(lines, cur, i+1, total)
        cur += tmp
    elif line.startswith("$ cd .."):
        return total + cur, i+1

solve(data)
