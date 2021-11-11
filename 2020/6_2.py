import numpy as np


x = open("data6.txt","r").read().split("\n\n")

def solve(x):

    count = 0

    for group in x:
        

        group = group.split("\n")
        if "" in group:
            group.remove("")

        ppl = len(group)

        g = "".join(group)
        mem = []
        while g != "":
            for letter in g:
                if g.count(letter) == ppl:
                    count +=1
                g = g.replace(letter,"")
            
    print(count)
solve(x)
