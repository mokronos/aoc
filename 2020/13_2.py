x = open("data13.txt").read()


def solve(x):

    x = x.split("\n")
    buses = x[1].split(",")

    buses = [(int(buses[i]),i) for i in range(len(buses)) if buses[i] != "x"]
     
    count = 1
    time = 0
    for i in range(len(buses)-1):

        bus_id = buses[i+1][0]
        idx = buses[i+1][1]
        count *= buses[i][0]

        while (time + idx) % bus_id != 0:
            time += count

    print(time)
solve(x)


#credit: https://www.reddit.com/r/adventofcode/comments/kc4njx/2020_day_13_solutions/gfth69h?utm_source=share&utm_medium=web2x&context=3
