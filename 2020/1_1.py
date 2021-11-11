import numpy as np


y = np.loadtxt("data1.txt", dtype = int)

def solve(x):

    for i in x:
        for j in x:
            if i+j == 2020:
                print(i*j)
                return


solve(y)
