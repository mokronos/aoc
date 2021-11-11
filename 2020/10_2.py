import numpy as np
import time

x = np.loadtxt("data10.txt", dtype = int)

def solve(x):
    x = list(x) 
    x.sort()
    x.insert(0,0)
    x.append(x[-1] +3)

    mem = [0]*3
    oldmem = [0]*3
    oldmem[0] = 1
    
    for i in range(x[-1],-1,-1):
        if i in x:
            mem[1:] = oldmem[:2]
            mem[0] = 0
            for j in range(len(mem)):
                mem[0] += oldmem[j]
        else:
            mem[0] = 0
            mem[1:] = oldmem[:2]

        oldmem[:] = mem[:]
    print(mem[0])
solve(x)
