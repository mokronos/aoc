data = open("data13.txt", "r").read()

def solve(data):
    packets = data.split("\n\n")
     
    packs = []
    for idx, packet in enumerate(packets):
        left = eval(packet.splitlines()[0])
        right = eval(packet.splitlines()[1])
        packs.append(left)
        packs.append(right)

    packs.append([[2]])
    packs.append([[6]])

    for i in range(len(packs)):
        for j in range(len(packs)-i-1):

            left, right = packs[j], packs[j+1]
            # print seperator
            # print("-"*20)
            c = check(left, right)
            # print(f"check: {c}")
            if c in [1, 2]:
                # print(f"packet {j} is smaller than packet {j+1}")
                pass
            else:
                # print(f"packet {j} is larger than packet {j+1}")
                packs[j], packs[j+1] = packs[j+1], packs[j]
            # print seperator
            # print("-"*20)

    key1 = packs.index([[2]]) + 1
    key2 = packs.index([[6]]) + 1 

    for i, pack in enumerate(packs):
        print(f"packet {i}: {pack}")

    print(key1*key2)

def check(left, right):
    """
    return True if left is smaller than right
    else return False
    """

    # print(f"comparing {left} and {right}")
    
    len_left = len(left)
    len_right = len(right)
    length = min(len_left, len_right)

    for i in range(length):

        l = left[i]
        r = right[i]

        if type(l) == list and type(r) == list:
            c = check(l, r)
            if c == 0:
                return 0
            elif c == 1:
                continue
            elif c == 2:
                return 2
        if type(l) != list and type(r) == list:
            l = [left[i]]
            c = check(l, r)
            if c == 0:
                return 0
            elif c == 1:
                continue
            elif c == 2:
                return 2

        if type(l) == list and type(r) != list:
            r = [r]
            c = check(l, r)
            if c == 0:
                return 0
            elif c == 1:
                continue
            elif c == 2:
                return 2

        if l > r:
            return 0
        elif l == r:
            continue
        elif l < r:
            return 2
        
    if len_left > len_right:
        return 0
    elif len_left == len_right:
        return 1
    elif len_left < len_right:
        return 2
            

solve(data)
