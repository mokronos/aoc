import numpy as np


x = np.loadtxt("data5.txt", dtype = str)

def solve(x):

    solution = 0

    for string in x:
        rows = string[0:7]
        columns = string[-3:]
        rows = rows.replace("B","1").replace("F","0")
        columns = columns.replace("R","1").replace("L","0")

        rows = int(rows,2)
        columns = int(columns,2)

        pass_id = rows*8 + columns

        if pass_id > solution:
            solution = pass_id
    print(solution)


solve(x)

