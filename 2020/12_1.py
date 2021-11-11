
x = open("data12.txt").read()
x = x.split("\n")
x = x[:-1]

#N,W,S,E
p = {"N":0,"W":0,"S":0,"E":0}
d_names = ["N","W","S","E"]
d = 3
for line in x:
    
    if line[0] == "L":
        d +=1*int(line[1:])//90
        d %=4
    elif line[0] == "R":
        d +=3*int(line[1:])//90
        d %=4
    elif line[0] == "F":

        p[d_names[d]] +=int(line[1:])

    else:
        p[line[0]] += int(line[1:])


dist = abs(p["N"]-p["S"]) + abs(p["W"]-p["E"])
print(dist)
