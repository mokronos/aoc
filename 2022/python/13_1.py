from ast import literal_eval as eval
data = open("data13.txt", "r").read()

def solve(data):
    packets = data.split("\n\n")
     


    result = 0
    for idx, packet in enumerate(packets):
        left = eval(packet.splitlines()[0])
        right = eval(packet.splitlines()[1])

        print(f"left: {left}")
        print(f"right: {right}")

        # print seperator
        print("-"*20)
        c = check(left, right)
        print(f"check: {c}")
        if c in [1, 2]:
            print(f"packet {idx+1} IS in right order")
            result += idx+1
        else:
            print(f"packet {idx+1} is NOT in the right order")
        # print seperator
        print("-"*20)

    print(result)

def check(left, right):
    """
    return True if left is smaller than right
    else return False
    """

    print(f"comparing {left} and {right}")
    
    len_left = len(left)
    len_right = len(right)
    length = min(len_left, len_right)

    for i in range(length):

        if type(left[i]) == list and type(right[i]) == list:
            c = check(left[i], right[i])
            if c == 0:
                return 0
            elif c == 1:
                continue
            elif c == 2:
                return 2
        if type(left[i]) != list and type(right[i]) == list:
            left[i] = [left[i]]
            c = check(left[i], right[i])
            if c == 0:
                return 0
            elif c == 1:
                continue
            elif c == 2:
                return 2

        if type(left[i]) == list and type(right[i]) != list:
            right[i] = [right[i]]
            c = check(left[i], right[i])
            if c == 0:
                return 0
            elif c == 1:
                continue
            elif c == 2:
                return 2

        if left[i] > right[i]:
            return 0
        elif left[i] == right[i]:
            continue
        elif left[i] < right[i]:
            return 2
        
    if len_left > len_right:
        return 0
    elif len_left == len_right:
        return 1
    elif len_left < len_right:
        return 2
            

solve(data)
