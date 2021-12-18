x = open("data18.txt").read()
def clean(x):
    x = x.split("\n")[:-1]
    res = []
    for line in x:
        line = [i for i in line if i != " "]
        res.append(line)
    return res

def solve(eq):
    if len(eq) == 1:
        return eq

    elif "(" in eq:
        end = eq.index(")")
        rev = list(reversed(eq))
        start = len(eq) - rev.index("(",len(eq)-end-1) - 1
        eq = eq[:start] + solve(eq[start+1:end]) + eq[end+1:]
        return solve(eq)

    else:
        if "+" in eq:
            start = eq.index("+") - 1
            end = start + 2
            eq = eq[:start] + [str(eval("".join(eq[start:end+1])))] + eq[end+1:]
            return solve(eq)
        else:
            x = eval("".join(eq[0:3]))
            return solve([str(x)]+eq[3:])

def final(x):
    result = 0
    for item in x:
        result += int(solve(item)[0])
    print(result)


x = clean(x)
final(x)
