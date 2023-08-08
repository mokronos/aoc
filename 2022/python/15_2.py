import re

data = open("data15.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    sensors, beacons = parse(lines)
    limit = 4000000
    # limit = 20
    xrange = [0,limit]
    yrange = [0,limit]
    candidates = get_candidates(xrange, yrange, sensors, beacons)
    result = check(candidates, sensors, beacons)
    print(f"Found the position: {result}")
    print(f"tuning frequency: {result[0]*limit + result[1]}")

def get_candidates(xrange, yrange, sensors, beacons):
    candidates = []

    for s,b in zip(sensors,beacons):
        sx,sy = s
        bx,by = b
        dist = abs(sx-bx) + abs(sy-by) + 1

        maxx = sx + dist
        minx = sx - dist
        maxy = sy + dist
        miny = sy - dist

        # create diamond shape
        top_right = []
        for x,y in zip(range(sx, maxx), range(miny, sy)):
            if not (xrange[0] <= x <= xrange[1] and yrange[0] <= y <= yrange[1]): continue
            top_right.append((x,y))
        right_bottom = []
        for x,y in zip(range(sx, maxx), range(sy, maxy)):
            if not (xrange[0] <= x <= xrange[1] and yrange[0] <= y <= yrange[1]): continue
            right_bottom.append((x,y))
        bottom_left = []
        for x,y in zip(range(minx, sx), range(sy, maxy)):
            if not (xrange[0] <= x <= xrange[1] and yrange[0] <= y <= yrange[1]): continue
            bottom_left.append((x,y))
        left_top = []
        for x,y in zip(range(minx, sx), range(miny, sy)):
            if not (xrange[0] <= x <= xrange[1] and yrange[0] <= y <= yrange[1]): continue
            left_top.append((x,y))

        candidates += top_right + right_bottom + bottom_left + left_top

        print(f"created diamond shape for sensor {s} and beacon {b}")

    return candidates
def check(candidates, sensors, beacons):
    print(f"checking {len(candidates)} candidates")
    for idx, (x,y) in enumerate(candidates):
        # print status every 10000th candidate
        if idx % 100000 == 0: print(f"checking candidate {idx} of {len(candidates)}")
        c = True
        for s,b in zip(sensors,beacons):
            dist = abs(s[0]-b[0]) + abs(s[1]-b[1])
            dist_alt = abs(s[0]-x) + abs(s[1]-y)
            if dist_alt <= dist:
                c = False
                break
        if c:
            return (x,y)

def parse(lines):
    match = "-?[0-9]+"
    sensors = []
    beacons = []
    for line in lines:
        sx,sy,bx,by = re.findall(match, line)
        sensors.append((int(sx),int(sy)))
        beacons.append((int(bx),int(by)))

    return sensors, beacons
solve(data)
