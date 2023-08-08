data = open("data1.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    max_cal = [0,0,0]
    cals = 0
    for line in lines:

        if line == "":
            if cals>max_cal[0]:
                max_cal[0] = cals
                max_cal.sort()
                cals = 0
            else:
                cals = 0
        else:

            cals += int(line)

    print(max_cal)
    print(sum(max_cal))
solve(data)
