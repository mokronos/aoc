from z3 import Optimize, Int, Sum, sat
from typing import List, Tuple

with open("data10.txt", "r") as f:
    RAW = f.read().strip()

# RAW = """
# [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
# [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
# [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
# """.strip()

def parse_line(line: str) -> Tuple[List[bool], List[List[int]], List[int]]:
    parts = line.split()
    first = parts[0]
    middle = parts[1:-1]
    last = parts[-1]

    lights = [c == "#" for c in first[1:-1]]

    wirings = []
    for w in middle:
        inner = w[1:-1]
        if inner == "":
            wirings.append([])
        else:
            wirings.append([int(x) for x in inner.split(",")])

    joltage = [int(x) for x in last[1:-1].split(",")] if last[1:-1] != "" else []

    return lights, wirings, joltage

def solve_joltage(joltage: List[int], wirings: List[List[int]]) -> Tuple[int, List[int]]:

    m = len(wirings)
    n = len(joltage)
    R = [[0] * n for _ in range(m)]
    for i in range(m):
        for col in wirings[i]:
            R[i][col] = 1

    opt = Optimize()

    x = [Int(f"x_{i}") for i in range(m)]
    for xi in x:
        opt.add(xi >= 0)

    for j in range(n):
        terms = []
        for i in range(m):
            if R[i][j] != 0:
                if R[i][j] == 1:
                    terms.append(x[i])
                else:
                    terms.append(x[i] * R[i][j])
        if terms:
            opt.add(Sum(*terms) == joltage[j])
        else:
            if joltage[j] != 0:
                return None, []

    # minimize total presses
    total_ops = Sum(*x) if x else Int("zero_placeholder")
    if x:
        opt.minimize(total_ops)

    res = opt.check()
    if res != sat:
        return None, []

    model = opt.model()
    sol = [model.eval(xi).as_long() for xi in x]
    total = sum(sol)
    return total, sol

def main():
    lines = [ln.strip() for ln in RAW.splitlines() if ln.strip()]
    manuals = [parse_line(ln) for ln in lines]

    grand_total = 0
    for _, (_, wirings, joltage) in enumerate(manuals):
        result, _ = solve_joltage(joltage, wirings)
        grand_total += result

    print(grand_total)

if __name__ == "__main__":
    main()
