import numpy as np


x = np.loadtxt("data5.txt", dtype = str)

def solve(x):

    mem = []
    for string in x:
        rows = string[0:7]
        columns = string[-3:]
        rows = rows.replace("B","1").replace("F","0")
        columns = columns.replace("R","1").replace("L","0")

        rows = int(rows,2)
        columns = int(columns,2)

        pass_id = rows*8 + columns

        mem.append(pass_id)
    
    mem.sort()

    for i in range(1,mem[-1]-1, 1):

        if (i-1 in mem) and (i+1 in mem) and (i not in mem):

            print(i)



solve(x)

