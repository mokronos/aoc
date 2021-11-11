x = open("data15.txt").read().split("\n")[0].split(",")
def solve(x):
    
    x = [int(i) for i in x]

    while len(x)< 2020:
        
        if x[-1] not in x[:-1]:
            x.append(0)
        else:
            for i in range(len(x)-1):
                if x[-1] == x[i]:
                    idx = i
            x.append(len(x)-idx -1)


    print(x[-1])

solve(x)
