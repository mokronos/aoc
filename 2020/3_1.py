import numpy as np

y = open("data3.txt")
y = y.read()

x = np.loadtxt("data3.txt", dtype = str, comments = "6", delimiter = "\n")


def solve(x):

    tree_count = 0

    pos_x = 0
    pos_y = 0

    while pos_y<len(x):

        rel_x = pos_x % len(x[0])

        if x[pos_y][rel_x] == "#":
            tree_count += 1
        
        pos_x += 3
        pos_y += 1
    
    print(tree_count)

solve(x)
