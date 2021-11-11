x = open("data15.txt").read().split("\n")[0].split(",")
def solve(x):


    
    x = [int(i) for i in x]
    memory = {}
    for i in range(len(x)-1):
        memory[x[i]] = i
    last_value = x[-1]
    for i in range(5,30000000 -1):
        
        if memory.get(last_value) == None:
            memory[last_value] = i
            last_value = 0
        else:

            last_index = memory.get(last_value)
            memory[last_value] = i
            last_value = i - last_index


    print(last_value)

solve(x)
