import numpy as np

x = np.loadtxt("data4.txt", dtype = str, comments = "$", delimiter = "\n\n")


y = open("data4.txt", "r").read()

def solve(x):

    count = 0
    y_split = y.split("\n\n")

    for i in range(len(y_split)):
        y_split[i] = y_split[i].replace(" ","\n")
        y_split[i] = y_split[i].split("\n")

        if len(y_split[i]) > 7:
            count +=1
        
        if len(y_split[i]) == 7:
            cid_found = 0
            for i in y_split[i]:

                if i[0:3] == "cid":

                    cid_found = 1

            if cid_found == 0:
                count +=1

    print(count)

solve(y)
