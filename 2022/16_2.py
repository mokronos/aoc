import re
import collections
import functools
import itertools
data = open("data16.txt", "r").read()


def solve(data):

    r = r"Valve ([A-Z]{2}).*=(\d+).*valves? (.*)"
    f = re.findall(r, data)

    valves, flows, dists = set(), dict(), collections.defaultdict(lambda:1000)

    for v, f, e in f:
        valves.add(v)
        if f != "0": flows[v] = int(f)
        for d in e.split(", "):
            dists[v, d] = 1
        
    for k,i,j in itertools.product(valves, valves, valves):
        dists[i,j] = min(dists[i,j], dists[i,k] + dists[k,j])


    @functools.lru_cache(None)
    def search(t, node = "AA", vs=frozenset(flows), e = False):
        tmp = []
        for v in vs:
            if dists[node,v] < t:
                water = flows[v] * (t - dists[node,v]-1)
                water_extra = search(t - dists[node,v]-1, v, vs - {v}, e)
                total = water + water_extra
                tmp.append(total)
        tmp.append(search(26,vs=vs) if e else 0)
        return max(tmp)

    result = search(26, e=True)
    print(result)

solve(data)
