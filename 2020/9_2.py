import numpy as np


x = np.loadtxt("data9.txt")

def solve(x):
    
    for i in range(25,len(x)):

        mem = x[i-25:i]
        goal = x[i]
        valid = 0
        for number in mem:

            for number2 in mem:

                if number == number2:
                    continue

                if (number + number2) == goal:
                    valid = 1
        if valid == 0:
            return(goal)


def solve2(x,goal):

    for i in range(len(x)):

        for j in range(i,len(x)):
            mem = x[i:j]

            if sum(mem) == goal:

                mem.sort()
                solution = mem[0] + mem[-1]
                print(solution)
                return



    

goal1 = solve(x)

solve2(x,goal1)

