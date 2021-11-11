import numpy as np

x = np.loadtxt("data2.txt", str)

def solve(x):
    password_count = 0

    for qt,value,password in x:

        

        qt_split = list(map(int, qt.split("-")))

        count = 0
        for letter in password:

            if letter == value[0]:
                count += 1

       
        if qt_split[0] <= count <= qt_split[1]:
            password_count += 1

    print(password_count)
        
solve(x)
