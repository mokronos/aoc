import numpy as np

x = np.loadtxt("data10.txt", dtype = int)

def solve(x):
    x = list(x) 
    x.sort()
    x.append(x[-1] +3)
    x.insert(0,0)

    count1 = 0
    count3 = 0
    for i in range(0, len(x)-1):
        if x[i+1]-x[i] == 1:
            count1 +=1

        if x[i+1]-x[i] == 3:
            count3 +=1

    print(count3*count1)
solve(x)
