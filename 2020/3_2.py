import numpy as np

y = open("data3.txt")
y = y.read()

x = np.loadtxt("data3.txt", dtype = str, comments = "6", delimiter = "\n")


def solve(x):

    tree_count = np.zeros(5, dtype = int)

    pos_x = np.zeros(5, dtype = int)
    pos_y = np.zeros(5, dtype = int)
    print(pos_y)

    slope_x = [1,3,5,7,1]
    slope_y = [1,1,1,1,2]
    
    for i in range(5):

        while pos_y[i]<len(x):

            rel_x = pos_x[i] % len(x[0])



            print(x[pos_y[i]])

            if x[pos_y[i]][rel_x] == "#":
                tree_count[i] += 1
            
            pos_x[i] += slope_x[i]
            pos_y[i] += slope_y[i]
        
    tree_count = tree_count.astype("float32")
    tree_count_mult = 1 
    for i in range(5):

        tree_count_mult *= tree_count[i]


    print(np.prod(tree_count))

    print(tree_count)
    print(tree_count_mult)

solve(x)
