import re
with open("data21.txt", "r") as f:
    data = f.read().strip()

def solve(data):

    seen = dict()
    q = dict()
    ops = {"+": lambda x, y: x + y, "-": lambda x, y: x - y, "*": lambda x, y: x * y, "/": lambda x, y: x / y}
    for line in data.splitlines():
        r = r"(.*): (.*) ([\+\-\/\*]) (.*)"
        if x := re.findall(r, line):
            x = x[0]
            # if x[0] == "root":
            #     continue
            q[x[0]] = (x[1], x[2], x[3])
        else:
            x = line.split(": ")
            seen[x[0]] = int(x[1])

    def search(seen, node, found = 0):
        if node == "humn":
            found = 1
        if node in seen:
            return seen[node], found, seen
        else:
            x = q[node]
            x0, found, seen = search(seen, x[0], found)
            x2, found, seen = search(seen, x[2], found)
            func = ops[x[1]]
            seen[node] = func(x0, x2)

            return seen[node], found, seen

    def back(seen, start, end = "humn"):

        if start == end:
            return seen[start]

        x = q[start]
        node1 = x[0]
        node2 = x[2]
        
        lv, lc,_ = search(seen.copy(), node1)
        rv, rc,_ = search(seen.copy(), node2)

        sval = seen[start]
        val = rv if lc else lv
        new_start = node1 if lc else node2

        if x[1] == "+":
            seen[new_start] = sval - val

        elif x[1] == "*":
            seen[new_start] = sval / val

        elif x[1] == "-":
            if lc:
                # left side is x
                # sval = x - val
                seen[new_start] = sval + val
            else:
                # right side is x
                # sval = val - x
                seen[new_start] = val - sval

        elif x[1] == "/":
            if lc:
                # left side is x
                # sval = x / val
                seen[new_start] = sval * val
            else:
                # right side is x
                # sval = val / x
                seen[new_start] = val / sval

        return back(seen, new_start)


    node1 = q["root"][0]
    node2 = q["root"][2]
    
    # check which side of the root the humn node is in?
    lv, lc,_ = search(seen.copy(), node1)
    rv, rc,_ = search(seen.copy(), node2)

    # calculate the side without the humn node
    val = rv if lc else lv
    start = node1 if lc else node2

    print(f"need to get val for humn, so that {start} = {val}")

    # search backward from other node, check at every fork where humn is, calculate other side to calculate needed value

    seen[start] = val
    result = back(seen.copy(), start)
    print(f"result: {int(result)}")

solve(data)
