with open("data25.txt") as f:
    data = f.read().strip()



def to10(a, exp = 5):
    a = [-1 if x == "-" else -2 if x == "=" else int(x) for x in a]

    res = 0
    for i, char in enumerate(a[::-1]):
        res += char * exp ** i
    return res

def to5(a, exp = 5):

    res = []
    while a > 0:
        res.append(a % exp)
        a = a // exp
    result = []
    rem = 0
    for val in res:
        val = val + rem
        if val <= 2:
            result.append(val)
            rem = 0
        else:
            result.append(0 if val == 5 else -1 if val == 4 else -2)
            rem = 1
    if rem == 1:
        result.append(1)

    result = ["-" if x == -1 else "=" if x == -2 else str(x) for x in result]
    return "".join(result[::-1])

def solve(data):
    data = data.splitlines()

    snafu = []
    for line in data:
        snafu.append(to10(line))

    total = sum(snafu)
    
    print(f"total: {total}")
    print(f"total in base 5: {to5(total)}")

solve(data)
