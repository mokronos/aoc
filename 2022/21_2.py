import re
with open("data21.txt", "r") as f:
    data = f.read().strip()

def solve(data):

    seen = dict()
    q = dict()
    for line in data.splitlines():
        r = r"(.*): (.*) ([\+\-\/\*]) (.*)"
        if x := re.findall(r, line):
            x = x[0]
            q[x[0]] = (x[1], x[2], x[3])
        else:
            x = line.split(": ")
            seen[x[0]] = int(x[1])


    def search(node):
        if node in seen:
            return seen[node]
        else:
            x = q[node]
            if node == "root":
                return search(x[0]) == search(x[2])
            elif x[1] == "+":
                seen[node] = search(x[0]) + search(x[2])
            elif x[1] == "-":
                seen[node] = search(x[0]) - search(x[2])
            elif x[1] == "*":
                seen[node] = search(x[0]) * search(x[2])
            elif x[1] == "/":
                seen[node] = search(x[0]) / search(x[2])
            return seen[node]
    
    org_seen = seen.copy()
    org_q = q.copy()
    for i in range(200000):
        seen = org_seen.copy()
        q = org_q.copy()
        seen["humn"] = i
        if search("root"):
            print(i)
solve(data)
