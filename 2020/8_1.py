import numpy as np

x = np.loadtxt("data8.txt", dtype = str)

def solve(x):

    acc = 0
    idx = 0
    idx_mem = []
    while(1):



        if idx in idx_mem:

            print(acc)
            return

        idx_mem.append(idx)

        if x[idx,0] == "acc":
            acc += int(x[idx,1])
            idx += 1
        if x[idx,0] == "nop":
            idx += 1

        if x[idx,0] == "jmp":

            idx += int(x[idx,1])

        

solve(x)
