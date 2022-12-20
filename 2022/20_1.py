with open("data20.txt", "r") as f:
    data = f.read().strip()

def solve(data):

    key = 1
    rounds = 1
    seq = [int(x)*key for x in data.splitlines()]
    idx = list(range(len(seq)))

    for i in idx*rounds:
        idx.pop(j:=idx.index(i))
        idx.insert((j+seq[i])%len(idx), i)
    
    zero_idx = idx.index(seq.index(0))

    result = sum(seq[idx[(zero_idx+pos)%len(seq)]] for pos in [1000, 2000, 3000])

    print(result)

solve(data)
