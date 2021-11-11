import numpy as np


x = open("data6.txt","r").read().split("\n\n")

def solve(x):

    count = 0

    for group in x:

        group = group.replace("\n", "")

        mem = []

        for letter in group:
            if letter not in mem: 
                mem.append(letter)
        
        count += len(mem)
    print(count)
solve(x)
