import re


x = open("data16.txt").read()

x = x.split("\n\n")
x = [x.split("\n") for x in x]

rules = x[0]
myticket = x[1][1:]
ntickets = x[2][1:-1]

def clean_rules(rules):

    pat_nmb = re.compile(r"\w+: (\d+)-(\d+) or (\d+)-(\d+)")
    pat_desc = re.compile(r"(\w+): \d+-\d+ or \d+-\d+")
    res = []
    for rule in rules:
        r = []
        nmb = pat_nmb.findall(rule)
        nmb = list(map(int,nmb[0]))
        desc = pat_desc.findall(rule)
        r.append(desc[0])
        r += nmb
        
        res.append(r)

    return res

rules = clean_rules(rules)

clean_t = lambda myticket : list(map(int,myticket.split(",")))
myticket = clean_t(myticket[0])
ntickets = [clean_t(ticket) for ticket in ntickets]

def check(nmb, rule):

    if nmb<rule[1] or nmb>rule[4] or (nmb>rule[2] and nmb<rule[3])  :
        return False
    else:
        return True

err = 0
for ticket in ntickets:
    for nmb in ticket:
        valid = 0
        for rule in rules:
            if check(nmb, rule):
                valid = 1
                break
        if not valid:
            err += nmb

print(err)
