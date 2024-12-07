package main

import (
	"aoc/internal"
	"fmt"
    "strings"
)

func indexOf[T comparable](arr []T, target T) int {
    for i := 0; i < len(arr); i++ {
        if arr[i] == target {
            return i
        }
    }
    return -1
}


type Coord struct {
    x int
    y int
}



func part1() {
    content := internal.ReadInput("data/data6.txt")
    res := strings.Split(content, "\n")

    var grid [][]string

    var start = Coord{0, 0}

    for i, line := range res {
        if line == "" {
            continue
        }

        var res = strings.Split(line, "")

        j := indexOf(res, "^")

        if j != -1 {
            start = Coord{j, i}
        }
        grid = append(grid, res)
    }

    var pos = start
    var dirs = [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }
    var dir = 2

    var visited = []Coord{}
    visited = append(visited, pos)

    for {
        var d = dirs[dir]
        var newPos = Coord{pos.x + d.x, pos.y + d.y}
        if newPos.x < 0 || newPos.x >= len(grid[0]) || newPos.y < 0 || newPos.y >= len(grid) {
            break
        }

        if grid[newPos.y][newPos.x] == "." || grid[newPos.y][newPos.x] == "^" {
            pos = newPos
            if indexOf(visited, pos) == -1 {
                visited = append(visited, pos)
            }
        } else if grid[newPos.y][newPos.x] == "#" {
            dir = (dir + 1) % 4
        }


    }

    var total = 0

    total = len(visited)

    fmt.Println("Part 1: ", total)
}

type State struct {
    pos Coord
    dir int
}

func is_loop(grid [][]string, start Coord) bool {

    var pos = start
    var dirs = [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }
    var dir = 2

    var visited = map[State]bool{}
    visited[State{pos, dir}] = true

    for {
        var d = dirs[dir]
        var newPos = Coord{pos.x + d.x, pos.y + d.y}
        if newPos.x < 0 || newPos.x >= len(grid[0]) || newPos.y < 0 || newPos.y >= len(grid) {
            break
        }

        if grid[newPos.y][newPos.x] == "." || grid[newPos.y][newPos.x] == "^" {
            pos = newPos
            state := State{pos, dir}
            if _, ok := visited[state]; !ok {
                visited[state] = true
            } else {
                return true
            }

        } else if grid[newPos.y][newPos.x] == "#" {
            dir = (dir + 1) % 4
        }
    }
    return false
}

func part2() {
    content := internal.ReadInput("data/data6.txt")
    res := strings.Split(content, "\n")

    var grid [][]string

    var start = Coord{0, 0}

    for i, line := range res {
        if line == "" {
            continue
        }

        var res = strings.Split(line, "")

        j := indexOf(res, "^")

        if j != -1 {
            start = Coord{j, i}
        }
        grid = append(grid, res)
    }

    var total = 0

    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {

            fmt.Printf("Progress: %d/%d %d/%d\n", i, len(grid), j, len(grid[i]))

            if grid[i][j] == "^" || grid[i][j] == "#" {
                continue
            }

            grid[i][j] = "#"

            if is_loop(grid, start) {
                total++
            }
            grid[i][j] = "."
        }
    }

    fmt.Println("Part 2: ", total)
}


func main() {
    part1()
    part2()
}
