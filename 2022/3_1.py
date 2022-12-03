import string

data = open("data3.txt", "r").read()

def solve(data):

    lines = data.splitlines()
            
    lower = string.ascii_lowercase
    upper = string.ascii_uppercase

    prio = {}
    for idx, letter in enumerate(lower):
            prio[letter] = idx + 1

    for idx, letter in enumerate(upper):
            prio[letter] = 26 + idx + 1

    res = 0
    for line in lines:

        a, b = line[:len(line)//2], line[len(line)//2:]

        for letter in a:
            if letter in b:
                res += prio[letter]
                break

    
    print(res)

solve(data)
