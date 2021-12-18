import numpy as np

x = open("data17.txt").read()

def read_string(string):

    string = string.split()
    array = [] 
    for item in string:
        array.append(list(item))
    array = np.array([[array]])
    array = np.where(array == ".", 0, 1)

    return array

def check(array):

    if array[1,1,1,1]:
        if np.count_nonzero(array)-1 in [2,3]:
            return 1
        else:
            return 0
    else:
        if np.count_nonzero(array) == 3:
            return 1
        else:
            return 0

def pad(array, amount):
    s = array.shape
    s = tuple(x + amount*2 for x in s) 
    result = np.zeros((s[0],s[1],s[2],s[3]))
    result[amount:-amount,amount:-amount,amount:-amount,amount:-amount] = array
    return result

def cycle(array):

    array = pad(array, 2)
    carray = array.copy()

    for i in range(1, array.shape[0] - 1):
        for j in range(1, array.shape[1] - 1):
            for k in range(1, array.shape[2] - 1):
                for m in range(1, array.shape[2] - 1):
                    array[i,j,k,m] = check(carray[i-1:i+2,j-1:j+2,k-1:k+2,m-1:m+2])
    return array[1:-1,1:-1,1:-1,1:-1]

x = read_string(x)

for _ in range(6):
    x = cycle(x) 

active = np.count_nonzero(x)
print(active)
