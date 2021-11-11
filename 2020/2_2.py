import numpy as np

x = np.loadtxt("data2.txt", str)

def solve(x):
    password_count = 0

    for idx,value,password in x:

        idx_split = list(map(int, idx.split("-")))

        value = value[0]

        if idx_split[1] > len(password):
            continue

        if (password[idx_split[0]-1] == value[0]) != (password[idx_split[1]-1] == value[0]):

            password_count +=1


    print(password_count)
        
solve(x)
