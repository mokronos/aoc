import re
with open("data21.txt", "r") as f:
    data = f.read().strip()

def solve(data):

    seen = dict()
    q = dict()
    root1 = None
    root2 = None
    global c
    c = 0
    for line in data.splitlines():
        r = r"(.*): (.*) ([\+\-\/\*]) (.*)"
        if x := re.findall(r, line):
            x = x[0]
            if x[0] == "root":
                root1 = x[1]
                root2 = x[3]
                continue
            q[x[0]] = (x[1], x[2], x[3])
        else:
            x = line.split(": ")
            seen[x[0]] = int(x[1])


    def search(node):
        if node == "humn":
            global c
            c = 1
        if node in seen:
            return seen[node]
        else:
            x = q[node]
            if x[1] == "+":
                seen[node] = search(x[0]) + search(x[2])
            elif x[1] == "-":
                seen[node] = search(x[0]) - search(x[2])
            elif x[1] == "*":
                seen[node] = search(x[0]) * search(x[2])
            elif x[1] == "/":
                seen[node] = search(x[0]) / search(x[2])
            return seen[node]
    
    def get_path(node):
        if node == "humn":
            return "humn"
        if node in seen:
            return str(seen[node])
        else:
            x = q[node]

            path = get_path(x[0]) + " " + x[1] + " " + get_path(x[2])


    # check if one of the variables of root can be calculated
    org_seen = seen.copy()
    org_q = q.copy()

    search(root1)
    if c:
        to_search = root1
    else:
        to_search = root2

    seen = org_seen.copy()
    q = org_q.copy()

    target = search(root1 if root2 == to_search else root2)

    print("target", target)
    print("to_search", to_search)

    
    # search backwards for the value of to_search
    seen = org_seen.copy()
    q = org_q.copy()

    def backwards(node, target):

        if node == "humn":
            return

        if node in seen:
            return seen[node]
        else:
            x = q[node]
            if x[1] == "+":
                
            elif x[1] == "-":
                seen[node] = backwards(x[0], target) - backwards(x[2], target)
            elif x[1] == "*":
                seen[node] = backwards(x[0], target) * backwards(x[2], target)
            elif x[1] == "/":
                seen[node] = backwards(x[0], target) / backwards(x[2], target)
            return seen[node]

solve(data)
