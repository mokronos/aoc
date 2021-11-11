x = open("data14.txt").read().split("\n")

def solve(x):

    data = []

    for entry in x:

        entry = entry.split(" = ")
        if entry[0][:2] == "me":
            adress = int(entry[0].split("[")[1][:-1])
            value = int(entry[1])
            data.append([adress,bin(value)[2:]])
        elif entry[0][:2] == "ma":

            data.append(["m", entry[1]])
        
    mem = {} 
    for entry in data:



        if entry[0] == "m":

            mask = entry[1]
        else:

            entry[1] = format(int(entry[1],2),"036b")
            for i in range(len(entry[1])):

                if mask[i] == "X":
                    continue

                elif entry[1][i] == mask[i]:
                    continue
                else:
                    new_entry = list(entry[1])
                    new_entry[i] = str(int(not int(new_entry[i])))
                    entry[1] ="".join(new_entry)
            mem[entry[0]] = int(entry[1],2)
    print(sum(mem.values()))


solve(x)
