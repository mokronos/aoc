import re
import numpy as np


x = open("data16.txt").read()

x = x.split("\n\n")
x = [x.split("\n") for x in x]

rules = x[0]
myticket = x[1][1:]
ntickets = x[2][1:-1]

def clean_rules(rules):

    pat_nmb = re.compile(r"\w+ *\w*: (\d+)-(\d+) or (\d+)-(\d+)")
    pat_desc = re.compile(r"(\w+ *\w*): \d+-\d+ or \d+-\d+")
    res = []
    for rule in rules:
        r = []
        nmb = pat_nmb.findall(rule)
        nmb = list(map(int,nmb[0]))
        desc = pat_desc.findall(rule)
        print(desc)
        r.append(desc[0])
        r += nmb
        
        res.append(r)

    return res

rules = clean_rules(rules)

clean_t = lambda myticket : list(map(int,myticket.split(",")))
myticket = clean_t(myticket[0])
ntickets = [clean_t(ticket) for ticket in ntickets]

def check(nmb, rule):

    if nmb<rule[1] or nmb>rule[4] or (nmb>rule[2] and nmb<rule[3]):
        return False
    else:
        return True

err = 0
def get_invalid(ntickets, rules):
    res = []
    for idx, ticket in enumerate(ntickets):
        for nmb in ticket:
            valid = 0
            for rule in rules:
                if check(nmb, rule):
                    valid = 1
                    break
            if not valid:
                res.append(idx)
    return res

valid_tickets = ntickets.copy()
idxs =  get_invalid(ntickets,rules)
idxs.reverse()
for i in idxs:
    del valid_tickets[i]

valid_tickets = np.array(valid_tickets)
def solve(valid_tickets, rules):
    res = []
    while len(res) < valid_tickets.shape[1]:
        for i in range(valid_tickets.shape[1]):
            nmbs = valid_tickets[:,i]
            rules_valid = []
            for rule in rules:
                valid = 1
                for nmb in nmbs:
                    if not check(nmb,rule):
                        valid = 0
                        break
                if valid:
                    rules_valid.append(rule)
            if len(rules_valid) == 1:
                res.append(rules_valid[0] + [i])
                rules.remove(rules_valid[0])
    return res

fields = solve(valid_tickets,rules)

indices = []
for field in fields:
    if not re.search(r"departure",field[0])==None:
        indices.append(field[5])
result = 1
for i in indices:
    result *= myticket[i]
print(myticket)
print(result)

