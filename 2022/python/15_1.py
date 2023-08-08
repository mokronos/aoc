import re

data = open("data15.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    sensors, beacons = parse(lines)
    line_to_check = 2000000
    # line_to_check = 10

    positions = []
    for s,b in zip(sensors,beacons):
        print(f"checking s:{s} b:{b}")
        dist = abs(s[0]-b[0]) + abs(s[1]-b[1])
        xpos = s[0]
        while 1:
            dist_alt = abs(s[0]-xpos) + abs(s[1]-line_to_check)
            if dist_alt <= dist:
                positions.append((xpos,line_to_check))
            else:
                break
            xpos += 1
        xpos = s[0]

        while 1:
            dist_alt = abs(s[0]-xpos) + abs(s[1]-line_to_check)
            if dist_alt <= dist:
                positions.append((xpos,line_to_check))
            else:
                break
            xpos -= 1

    positions = list(set(positions))
    for beacon in beacons:
        if beacon in positions: positions.remove(beacon)
    
    print(f"Found {len(positions)} positions")


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
