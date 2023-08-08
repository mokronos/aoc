data = open("data14.txt", "r").read()

def solve(data):
    lines = data.splitlines()
    
    xmax = int(lines[0].split(" -> ")[0].split(",")[0])
    ymax = int(lines[0].split(" -> ")[0].split(",")[1])
    xmin = int(lines[0].split(" -> ")[0].split(",")[0])
    ymin = 0
    walls = []

    for line in lines:
        wall = line.split(" -> ")
        tmp = []
        for form in wall:
            x = int(form.split(",")[0])
            y = int(form.split(",")[1])
            tmp.append((x, y))
            xmax = max(x, xmax)
            ymax = max(y, ymax)
            xmin = min(x, xmin)
        walls.append(tmp)
    # print(walls)
    # print(f"X: {xmin} - {xmax}")
    # print(f"Y: {ymin} - {ymax}")
    xlen = xmax - xmin + 1
    ylen = ymax - ymin + 1
    
    # random high number (could calculate this, but too lazy)
    extrax = 200

    extray = 2
    grid = [["." for x in range(xlen+ extrax*2)] for y in range(ylen+extray)]
    grid[-1] = ["#" for x in range(xlen+ extrax*2)]
    for wall in walls:
        grid = spawn(grid, wall, xmin, extrax)

    sand_start = (500 - xmin + extrax, 0)
    # grid[sand_start[1]][sand_start[0]] = "+"
    # for x in grid:
    #     print(x)

    blocked = False
    sand_amount = 0
    while not blocked:
        grid, blocked = pour(grid, sand_start)
        # print(f"start blocked: {blocked}")
        sand_amount += 1 
        # for x in grid:
        #     print(x)
    print(f"sand_amount: {sand_amount}")
def pour(grid, sand_start):

    x_start, y_start = sand_start
    while 1:
        new_step = False
        new_y = y_start + 1

        new_x = [x_start, x_start-1, x_start+1]
        for x in new_x:
            if grid[new_y][x] == ".":
                y_start = new_y
                x_start = x
                new_step = True
                break
            elif grid[new_y][x] in ["#", "o"]:
                continue
        if not new_step:
            grid[y_start][x_start] = "o"
            if (x_start, y_start) == sand_start:
                return grid, True
            return grid, False

def spawn(grid, wall, xmin, extrax):
    for i in range(len(wall)-1):
        x1, y1 = wall[i]
        x2, y2 = wall[i+1]
        x1 -= xmin - extrax
        x2 -= xmin - extrax
        xrange = [x for x in range(min(x1, x2), max(x1, x2)+1)]
        yrange = [y for y in range(min(y1, y2), max(y1, y2)+1)]
        # print(f"x1: {x1}, x2: {x2}, xrange: {xrange}")
        # print(f"y1: {y1}, y2: {y2}, yrange: {yrange}")

        for x in xrange:
            for y in yrange:
                # print(f"({x}, {y})")
                grid[y][x] = "#"

    return grid
solve(data)
