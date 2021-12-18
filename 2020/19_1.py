import re
x = open("data19t.txt").read()

def clean(x):
    x = x.split("\n\n")
    rules = x[0]
    rules = rules.split("\n")
    data = x[1]
    
    pat = re.compile(r"([\d+|]){1,}")
    pat2 = re.compile(r"([\w+])")
    cleaned = []
    for line in rules:
        if ('"') in line:
            cleaned.append([[re.findall(pat2,line)[0]], [re.findall(pat2,line)[1]]])
        else:
            res = re.findall(pat,line)

    return cleaned, data

def geti(rules, idx):

    for index,line in enumerate(rules):
        if line[0] == idx:
            return index

def gen(rules):


    if len(rules)==1:
        return rules
        #look for rows with only letters --> substitute into all other rules --> delete row
        #should be left with rule 0


rules, data = clean(x)

print(rules)
