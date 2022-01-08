# credit to https://0xdf.gitlab.io/adventofcode2020/19#challenge
import re

x = open("data19.txt").read()

def gen_regex(r="0", depth = 15):
    if depth == 0:
        return ""
    if rules[r][0][0].startswith('"'):
        return rules[r][0][0].strip('"')
    return "(" + "|".join(["".join([gen_regex(sub, depth-1) for sub in subrule]) for subrule in rules[r]]) + ")"




rulesraw, data = x.split("\n\n")

rules = {}

for rule in rulesraw.split("\n"):
    num, r = rule.split(": ")
    rules[num] = [s.split() for s in r.split(" | ")]

r1 = re.compile(gen_regex())
print(r1)
res = [r1.fullmatch(sen) for sen in data.split("\n")]
print(len([x for x in res if x]))
