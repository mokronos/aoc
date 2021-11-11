import numpy as np
import copy
np.set_printoptions(threshold=np.inf)

x = np.loadtxt("data11.txt", dtype = str)

x = x.tolist()
for i in range(len(x)):
    x[i] = list(x[i][:])

def solve(x):

    y = copy.deepcopy(x)

    w = list(range(len(x)))
    h = list(range(len(x[0])))
    while(1):

        for i in range(len(x)):
            for j in range(len(x[0])):

                if x[i][j] == ".":
                    continue
                elif x[i][j] == "L":
                    count = 0
                    for k in range(1,-2,-1):
                        for l in range(1,-2,-1):
                            if i+k in w and j+l in h:
                                if x[i+k][j+l] == "#":
                                    count +=1
                    if count == 0:
                        y[i][j] = "#"

                elif x[i][j] == "#":
                    count = 0
                    for k in range(1,-2,-1):
                        for l in range(1,-2,-1):
                            if i+k in w and j+l in h:
                                if x[i+k][j+l] == "#":
                                    count +=1
                    if (count -1) >= 4:
                        y[i][j] = "L"
        if y == x:
            count = 0
            for i in range(len(x)):
                for j in range(len(x[0])):
                    if x[i][j] == "#":
                        count +=1
            return print(count)

        else:
            
            x = copy.deepcopy(y)

solve(x)

