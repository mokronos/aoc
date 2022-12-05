data = open("data5.txt", "r").read()

def solve(data):

    data = data.splitlines() 

    # get the line with number
    for idx,line in enumerate(data):
        if line[1].isnumeric():
            state_end = idx
            break
    
    # get the line with the state
    numbers = data[state_end]
    max_cols = int(numbers[-2])

    state = {str(x):[] for x in range(1,max_cols+1)}
    
    for row in range(state_end-1, -1, -1):
        for col in range(0,max_cols):
            if data[row][col*4+1] == " ":
                continue
            state[str(col+1)] += data[row][col*4+1]


    prnt(state)
    for line in data[state_end+2:]:
        amount, f, t = encode_rule(line)
        print(f"Move {amount} from {f} to {t}")

        # update state
        tmp = state[f][-amount:]
        state[f] = state[f][:-amount]
        state[t] += tmp
        prnt(state)

    res = [" " if len(x) == 0 else x[-1] for x in state.values()]
    res = "".join(res)
    print(res)

def encode_rule(line):

    rule = line.split(" ")

    return int(rule[1]), rule[3], rule[5]
    
def prnt(state):
    for k,v in state.items():
        print(f"{k}: {v}")

solve(data)
