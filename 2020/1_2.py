import numpy as np

y = np.loadtxt("data1.txt", dtype = int)

def solve(x):

    for i in x:
        for j in x:
            for k in x:
                if i+j + k == 2020:
                    print(i*j*k)
                    return


solve(y)
