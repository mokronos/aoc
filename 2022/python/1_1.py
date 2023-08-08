data = open("data1.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    max_cal = 0
    cals = 0
    for line in lines:

        if line == "":
            if cals>max_cal:
                max_cal = cals
                cals = 0
            else:
                cals = 0
        else:

            cals += int(line)

    print(max_cal)
solve(data)
