x = open("data14.txt").read().split("\n")

def combX(data):

    ads = [data]
    
    while(1):
        check = 0

        for line in ads:

            if "X" in line:
                check = 1
                
                ads.append(line.replace("X","0",1))
                ads.append(line.replace("X","1",1))
                ads.remove(line)
                break
        if check == 0:
            break
    return [int(i,2) for i in ads]


def solve(x):

    data = []

    for entry in x:

        entry = entry.split(" = ")
        if entry[0][:2] == "me":
            adress = int(entry[0].split("[")[1][:-1])
            value = int(entry[1])
            data.append([bin(adress)[2:],bin(value)[2:]])
        elif entry[0][:2] == "ma":

            data.append(["m", entry[1]])
        
    mem = {} 
    for entry in data:



        if entry[0] == "m":

            mask = entry[1]
        else:

            entry[0] = format(int(entry[0],2),"036b")

            new_entry = ""
            for i in range(len(entry[0])):


                if mask[i] == "0":
                    new_entry += entry[0][i]
                elif mask[i] == "1":
                    new_entry +="1"
                elif mask[i] == "X":
                    new_entry += "X"

            ads = combX(new_entry)
            for i in ads:

                mem[i] = int(entry[1],2)

    print(sum(mem.values()))


solve(x)
