import numpy as np

x = np.loadtxt("data7.txt", dtype = str, delimiter = "\n")

def search(start_bag, policy, count = 0, mem = []):
    
    for line in policy:
        if start_bag == line[0]:
            for bag in line[1:]:
                if bag == "":
                    mem = mem[:-1] 
                    return count, mem
                else:
                    mem.append(int(bag[0]))
                    print(mem)
                    count += np.prod(mem)
                    print(count)
                    count,mem = search(bag[1:],policy,count,mem)
            mem = mem[:-1]
            return count, mem
            
def solve(x):
    policy = []
    for idx, line in enumerate(x):
        
        main_bag = line.split("s contain")[0]
        main_bag = main_bag.replace(" ","")

        inner_bags = line.split("s contain ")[1]
        
        if inner_bags == "no other bags.":
            inner_bags = ""
        
        inner_bags = inner_bags.replace("bags","bag")
        inner_bags = inner_bags.replace(" ","")
        inner_bags = inner_bags.replace(".","")
        inner_bags = inner_bags.split(",")

        line = [main_bag] + inner_bags

        policy.append(line)
    count = 0
    count,mem = search("shinygoldbag",policy, count = 0)
    print(count) 
    print(mem)

solve(x)
