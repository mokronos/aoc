package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "math"
)

type Coord struct {
    x int
    y int
}

func find_path(grid [][]string, start Coord, end Coord) []Coord {

    q := []Coord{start}
    visited := map[Coord]bool{}
    visited[start] = true

    dirs := [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }

    path := []Coord{start}

    for len(q) > 0 {
        pos := q[0]
        q = q[1:]

        for _, d := range dirs {
            newPos := Coord{pos.x + d.x, pos.y + d.y}
            if newPos.x < 0 || newPos.x >= len(grid[0]) || newPos.y < 0 || newPos.y >= len(grid) {
                continue
            }

            if grid[newPos.y][newPos.x] == "#" {
                continue
            }

            if visited[newPos]{
                continue
            }

            path = append(path, newPos)

            if newPos == end {
                return path
            }

            q = append(q, newPos)
            visited[newPos] = true
        }
    }
    return []Coord{}
}

func find_cheats(base_path []Coord, min_time_save int, max_cheat_len int) int {
    total := 0
    for i, p := range base_path[:len(base_path) - min_time_save] {
        for j, q := range base_path[i + min_time_save:] {
            dist := int(math.Abs(float64(p.x - q.x)) + math.Abs(float64(p.y - q.y)))

            if dist <= max_cheat_len && dist <= j {
                total ++
            }
        }
    }
    return total
}

func part1() {
    content := internal.ReadInput("data/data20.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    grid := [][]string{}
    var start Coord
    var end Coord

    for i := 0; i < len(res); i++ {
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            if res[i][j] == byte('S') {
                start = Coord{j, i}
                line = append(line, string("."))
                continue
            }
            if res[i][j] == byte('E') {
                end = Coord{j, i}
                line = append(line, string("."))
                continue
            }
            line = append(line, string(org_line[j]))
        }
        grid = append(grid, line)
    }

    base_path := find_path(grid, start, end)

    min_time_save := 100
    max_cheat_len := 2

    total := find_cheats(base_path, min_time_save, max_cheat_len)

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data20.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    grid := [][]string{}
    var start Coord
    var end Coord

    for i := 0; i < len(res); i++ {
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            if res[i][j] == byte('S') {
                start = Coord{j, i}
                line = append(line, string("."))
                continue
            }
            if res[i][j] == byte('E') {
                end = Coord{j, i}
                line = append(line, string("."))
                continue
            }
            line = append(line, string(org_line[j]))
        }
        grid = append(grid, line)
    }

    base_path := find_path(grid, start, end)

    min_time_save := 100
    max_cheat_len := 20

    total := find_cheats(base_path, min_time_save, max_cheat_len)

    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
