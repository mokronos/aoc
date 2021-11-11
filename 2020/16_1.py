import re


x = open("data16.txt").read()

def solve(x):
    
    x1 = x.split("your ticket:")
    doc = x.split("nearby tickets:\n")[-1]
    doc = doc.split("\n")[:-1]
    pat = re.compile(r"([A-Za-z]* ?[A-Za-z]*): (\d+)-(\d+) or (\d+)-(\d+)")
    ruleset = pat.findall(x1[0])
    rules = {}
    for entry in ruleset:
        rules[entry[0]] = list(range(int(entry[1]),int(entry[2])+1)) + list(range(int(entry[3]),int(entry[4])+1))

    pat2 = re.compile(r"\d\d\d")

    tickets = []
    for line in doc:
        res = pat2.findall(line)
        res = list(map(int, res))
        tickets.append(res)

    error_rate = 0
    print(tickets)
    print(rules)
    for line in tickets:
        for entry in line:

            status = 0
            for rule in rules.values():

                if entry in rule:
                    status = 1
                    break
            
            if status == 0:
                error_rate += entry
    
    print(error_rate)
solve(x)

