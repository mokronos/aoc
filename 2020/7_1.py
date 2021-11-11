import numpy as np

x = np.loadtxt("data7.txt", dtype = str, delimiter = "\n")

def search(start_bag,policy, count = 0):
    
    for line in policy:
        if start_bag == line[0]:
            for bag in line[1:]:
                if bag == "shinygoldbag":
                    count += 1
                    return count, 0
                if bag == "":
                    return count, 1

                count, check = search(bag,policy,count)
                if check == 0:
                    return count, 0
            return count, 1

def solve(x):
    policy = []
    for idx, line in enumerate(x):
        
        main_bag = line.split("s contain")[0]
        main_bag = main_bag.replace(" ","")

        inner_bags = line.split("s contain ")[1]
        
        if inner_bags == "no other bags.":
            inner_bags = ""
        
        inner_bags = inner_bags.replace("bags","bag")
        for i in range(50):
            inner_bags = inner_bags.replace(str(i),"")
        
        inner_bags = inner_bags.replace(" ","")
        inner_bags = inner_bags.replace(".","")
        inner_bags = inner_bags.split(",")

        line = [main_bag] + inner_bags

        policy.append(line)
    count = 0 
    for i in policy:

        count,_ = search(i[0],policy, count)
    print(count)

solve(x)
