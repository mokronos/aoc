data = open("data7.txt", "r").read()

def solve(data):

    lines = data.splitlines()

    dirs = {}
    cwd = []

    i = 0
    while i < len(lines):

        if lines[i].startswith("$ cd .."):
            cwd.pop()
            i += 1
        elif lines[i].startswith("$ cd"):
            cwd.append(lines[i].split()[-1])
            i += 1
        elif lines[i].startswith("$ ls"):
            i += 1
            while not lines[i].startswith("$"):
                size = lines[i].split()[0]
                total += int(size)
                i += 1
            dirs["/".join(cwd)] = total

        print(cwd)
    print(dirs)

solve(data)
