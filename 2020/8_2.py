import numpy as np

x = np.loadtxt("data8.txt", dtype = str)

def solve(y):

    for i in range(y.shape[0]):

        x = np.copy(y)

        if y[i,0] == "acc":
            continue

        if y[i,0] == "jmp":

            x[i,0] = "nop"

        if y[i,0] == "nop":

            x[i,0] = "jmp"

        acc = 0
        idx = 0
        idx_mem = []
        while(1):
            if idx in idx_mem:
                break

            idx_mem.append(idx)

            if (idx < 0) or (idx > (y.shape[0]-1)):
                break

            if idx == y.shape[0]-1:
                if x[idx,0] == "acc":
                    acc += int(x[idx,1])
                    print(acc)
                    return
                if x[idx,0] == "nop":
                    print(acc)
                    return 

                if (x[idx,0] == "jmp") and (int(x[idx,1]) == 1):
                    print(acc)
                    return


            
            if x[idx,0] == "acc":
                acc += int(x[idx,1])
                idx += 1
            elif x[idx,0] == "nop":
                idx += 1

            elif x[idx,0] == "jmp":

                idx += int(x[idx,1])


            

solve(x)
