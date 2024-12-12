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


func get_heads(grid [][]int, start Coord, distinct bool) int {
    count := 0
    visited := map[Coord]bool{}

    queue := []Coord{start}

    dirs := [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }


    for len(queue) > 0 {
        pos := queue[0]
        queue = queue[1:]


        if grid[pos.y][pos.x] == 9 {
            count += 1
            continue
        }

        prev:= grid[pos.y][pos.x]

        for _, d := range dirs {
            newPos := Coord{pos.x + d.x, pos.y + d.y}
            if newPos.x < 0 || newPos.x >= len(grid[0]) || newPos.y < 0 || newPos.y >= len(grid) {
                continue
            }
            if grid[newPos.y][newPos.x] != prev+1 {
                continue
            }
            if !visited[newPos] || distinct {
                visited[newPos] = true
                queue = append(queue, newPos)
            }
        }

    }
    return count
}

func part1() {
    content := internal.ReadInput("data/data10.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    grid := [][]int{}

    total := 0

    for i := 0; i < len(res); i++ {

        line := []int{}
        linestr := strings.Split(res[i], "")

        for j := 0; j < len(linestr); j++ {
            num, _ := strconv.Atoi(linestr[j])
            line = append(line, num)
        }
        grid = append(grid, line)
    }

    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if grid[i][j] == 0 {
                heads := get_heads(grid, Coord{j, i}, false)
                total += heads
            }
        }
    }


    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data10.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    grid := [][]int{}

    total := 0

    for i := 0; i < len(res); i++ {

        line := []int{}
        linestr := strings.Split(res[i], "")

        for j := 0; j < len(linestr); j++ {
            num, _ := strconv.Atoi(linestr[j])
            line = append(line, num)
        }
        grid = append(grid, line)
    }

    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if grid[i][j] == 0 {
                heads := get_heads(grid, Coord{j, i}, true)
                total += heads
            }
        }
    }


    fmt.Println("Part 2: ", total)
}


func main() {
    part1()
    part2()
}
