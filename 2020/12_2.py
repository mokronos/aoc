
x = open("data12.txt").read()
x = x.split("\n")
x = x[:-1]

def rotate90(point):
    
    return [-point[1]*1, point[0]]
    
def solve(x):
    #N,W,S,E
    p = [0,0]
    wp = [10,1]
    d_names = ["N","W","S","E"]
    d = 3
    for line in x:
        
        if line[0] == "L":
            for i in range(int(line[1:])//90):
                wp = rotate90(wp)
        elif line[0] == "R":
            for i in range(3*int(line[1:])//90):
                wp = rotate90(wp)
        elif line[0] == "F":
            for i in range(len(p)):
                p[i] += wp[i]*int(line[1:])
        elif line[0] == "N":
            wp[1] += int(line[1:])
        elif line[0] == "W":
            wp[0] -= int(line[1:])
        elif line[0] == "S":
            wp[1] -= int(line[1:])
        elif line[0] == "E":
            wp[0] += int(line[1:])
            
    print(sum([abs(x) for x in p]))

solve(x)
