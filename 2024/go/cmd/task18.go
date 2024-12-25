package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "strconv"
)

type Coord struct {
    x int
    y int
}

type CoordScore struct {
    coord Coord
    score int
}

func find_shortest_path(bytes []Coord) int {
    grid := [71][71]string{}

    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            grid[i][j] = "."
        }
    }


    for i := 0; i < len(bytes); i++ {
        grid[bytes[i].y][bytes[i].x] = "#"
    }

    start := Coord{0, 0}

    q := []CoordScore{CoordScore{start, 1}}
    visited := [len(grid)][len(grid[0])]int{}
    for i := 0; i < len(visited); i++ {
        for j := 0; j < len(visited[i]); j++ {
            visited[i][j] = -1
        }
    }

    visited[start.y][start.x] = 0

    dirs := [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }

    for len(q) > 0 {
        pos_score := q[0]
        pos := pos_score.coord
        q = q[1:]

        for _, d := range dirs {
            newPos := Coord{pos.x + d.x, pos.y + d.y}
            if newPos.x < 0 || newPos.x >= len(grid[0]) || newPos.y < 0 || newPos.y >= len(grid) {
                continue
            }
            if grid[newPos.y][newPos.x] == "#" {
                continue
            }

            if visited[newPos.y][newPos.x] == -1 {
                visited[newPos.y][newPos.x] = visited[pos.y][pos.x] + 1
                q = append(q, CoordScore{newPos, visited[newPos.y][newPos.x]})
            } else {
                visited[newPos.y][newPos.x] = min(visited[newPos.y][newPos.x], visited[pos.y][pos.x] + 1)
            }
        }
    }

    return visited[len(grid) - 1][len(grid[0]) - 1]
}


func part1() {
    content := internal.ReadInput("data/data18.txt")
    res := strings.Split(content, "\n")

    bytes := []Coord{}

    for i := 0; i < len(res); i++ {
        if res[i] == "" {
            continue
        }
        line := strings.Split(res[i], ",")
        x, _ := strconv.Atoi(line[0])
        y, _ := strconv.Atoi(line[1])
        bytes = append(bytes, Coord{x, y})
    }

    steps := 1024

    total := 0
    total += find_shortest_path(bytes[:steps])

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data18.txt")
    res := strings.Split(content, "\n")

    bytes := []Coord{}

    for i := 0; i < len(res); i++ {
        if res[i] == "" {
            continue
        }
        line := strings.Split(res[i], ",")
        x, _ := strconv.Atoi(line[0])
        y, _ := strconv.Atoi(line[1])
        bytes = append(bytes, Coord{x, y})
    }

    steps := 1024

    var total Coord
    for i := steps; i < len(bytes); i++ {
        if find_shortest_path(bytes[:i]) == -1 {
            total = bytes[i-1]
            break
        }
    }

    fmt.Printf("Part 2: %d,%d\n", total.x, total.y)
}
func main() {
    part1()
    part2()
}
