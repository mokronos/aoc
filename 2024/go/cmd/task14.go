package main

import (
	"aoc/internal"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Coord struct {
    x int
    y int
}

type Robot struct {
    pos Coord
    v Coord
}

func moveAll(robots []Robot, x_max int, y_max int) {
    for i := 0; i < len(robots); i++ {
        new_x := robots[i].pos.x + robots[i].v.x
        new_y := robots[i].pos.y + robots[i].v.y

        if new_x < 0 {
            new_x += x_max
        }
        if new_y < 0 {
            new_y += y_max
        }
        robots[i].pos.x = new_x % x_max
        robots[i].pos.y = new_y % y_max
    }
}

func countRobots(robots []Robot, x_max int, y_max int) int {

    qs := [4]int{0,0,0,0}

    for _, robot := range robots {
        if robot.pos.x < x_max/2 && robot.pos.y < y_max/2 {
            qs[0] += 1
        }
        if robot.pos.x > x_max/2 && robot.pos.y < y_max/2 {
            qs[1] += 1
        }
        if robot.pos.x < x_max/2 && robot.pos.y > y_max/2 {
            qs[2] += 1
        }
        if robot.pos.x > x_max/2 && robot.pos.y > y_max/2 {
            qs[3] += 1
        }
    }

    return qs[0]*qs[1]*qs[2]*qs[3]
}

func visualize(robots []Robot, x_max int, y_max int) [][]string {
    grid := make([][]string, y_max)

    for i := 0; i < y_max; i++ {
        grid[i] = make([]string, x_max)
        for j := 0; j < x_max; j++ {
            grid[i][j] = "."
        }
    }

    for _, robot := range robots {
        if grid[robot.pos.y][robot.pos.x] == "." {
            grid[robot.pos.y][robot.pos.x] = "1"
        } else {
            cur_num, _ := strconv.Atoi(grid[robot.pos.y][robot.pos.x])
            cur_num += 1
            grid[robot.pos.y][robot.pos.x] = strconv.Itoa(cur_num)
        }
    }
    return grid
}

func create_binary_image(robots []Robot, x_max int, y_max int) [][]int {
    grid := make([][]int, y_max)

    for i := 0; i < y_max; i++ {
        grid[i] = make([]int, x_max)
        for j := 0; j < x_max; j++ {
            grid[i][j] = 0
        }
    }

    for _, robot := range robots {
        grid[robot.pos.y][robot.pos.x] = 1
    }
    return grid
}

func create_txt(path string) {
    file, err := os.Create(path)
    if err != nil {
        fmt.Println(err)
        return
    }
    file.Close()
}

func append_txt(path string, content string) {
    file, err := os.OpenFile(path, os.O_APPEND|os.O_WRONLY, 0644)
    if err != nil {
        fmt.Println(err)
        return
    }
    _, err = file.WriteString(content)
    if err != nil {
        fmt.Println(err)
        return
    }
    err = file.Close()
    if err != nil {
        fmt.Println(err)
        return
    }
}

func part1() {
    content := internal.ReadInput("data/data14.txt")
    lines := strings.Split(content, "\n")

    total := 0
    var robots []Robot
    for _, line := range lines {
        if line == "" {
            continue
        }

        re := regexp.MustCompile(`[+-]?\d+`)
        matches := re.FindAllString(line, -1)

        var robot Robot

        robot.pos.x, _ = strconv.Atoi(matches[0])
        robot.pos.y, _ = strconv.Atoi(matches[1])
        robot.v.x, _ = strconv.Atoi(matches[2])
        robot.v.y, _ = strconv.Atoi(matches[3])

        robots = append(robots, robot)
    }


    x_max := 101
    y_max := 103

    secs := 100

    for i := 0; i < secs; i++ {
        moveAll(robots, x_max, y_max)
    }

    total += countRobots(robots, x_max, y_max)
    fmt.Println("Part 1: ", total)
}

func variance(grid [][]int) float64 {

    sum_x, sum_y := 0, 0
    sum_x2, sum_y2 := 0, 0
    count := 0

    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if grid[i][j] == 1 {
                sum_x += j
                sum_y += i
                sum_x2 += j*j
                sum_y2 += i*i
                count += 1
            }
        }
    }

    if count == 0 {
        return 0
    }

    mean_x := float64(sum_x)/float64(count)
    mean_y := float64(sum_y)/float64(count)
    var_x := float64(sum_x2)/float64(count) - mean_x*mean_x
    var_y := float64(sum_y2)/float64(count) - mean_y*mean_y

    return var_x + var_y
}

func part2() {
    content := internal.ReadInput("data/data14.txt")
    lines := strings.Split(content, "\n")

    total := 0
    var robots []Robot
    for _, line := range lines {
        if line == "" {
            continue
        }

        re := regexp.MustCompile(`[+-]?\d+`)
        matches := re.FindAllString(line, -1)

        var robot Robot

        robot.pos.x, _ = strconv.Atoi(matches[0])
        robot.pos.y, _ = strconv.Atoi(matches[1])
        robot.v.x, _ = strconv.Atoi(matches[2])
        robot.v.y, _ = strconv.Atoi(matches[3])

        robots = append(robots, robot)
    }


    x_max := 101
    y_max := 103

    secs := 10000

    create_txt("tree.txt")

    append_txt("tree.txt", "Start:\n")
    g := create_binary_image(robots, x_max, y_max)
    for _, row := range g {
        s:= []string{}
        for _, col := range row {
            if col == 1 {
                s = append(s, "#")
            } else {
                s = append(s, ".")
            }
        }
        fmt.Println(strings.Join(s, ""))
    }

    min_var := -1.0
    tree_secs := 0

    for i := 0; i < secs; i++ {
        moveAll(robots, x_max, y_max)
        g := create_binary_image(robots, x_max, y_max)
        if min_var == -1.0 || variance(g) < min_var {
            min_var = variance(g)
            tree_secs = i + 1
            fmt.Println("Min variance: ", min_var, " after ", i+1, " seconds")
            for _, row := range g {
                s:= []string{}
                for _, col := range row {
                    if col == 1 {
                        s = append(s, "#")
                    } else {
                        s = append(s, ".")
                    }
                }
                fmt.Println(strings.Join(s, ""))
            }
        }
    }

    total += tree_secs
    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
