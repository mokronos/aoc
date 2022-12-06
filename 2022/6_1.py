data = open("data6.txt", "r").read()

# data = "bvwbjplbgvbhsrlpgdmjqwftvncz"
def solve(data):

    print(data)
    n = 4
    for i, letter in enumerate(data):

        if i < n:
            continue

        if len(set(data[i-n:i])) == n:
            print(data[i-n:i])
            print(i)
            break


solve(data)
