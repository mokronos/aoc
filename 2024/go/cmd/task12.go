package main

import (
	"aoc/internal"
	"fmt"
    "strings"
)

type Coord struct {
    x int
    y int
}

func check_corner(grid [][]string, start Coord, plot_type string) int {
    corners := 0
    dirs := [4][3]Coord{
        {
            {0, -1},
            {-1, 0},
            {-1, -1},
        },
        {
            {1, 0},
            {0, -1},
            {1, -1},
        },
        {
            {0, 1},
            {1, 0},
            {1, 1},
        },
        {
            {-1, 0},
            {0, 1},
            {-1, 1},
        },
    }

    for _, d := range dirs {
        top_coord := Coord{start.x + d[0].x, start.y + d[0].y}
        left_coord := Coord{start.x + d[1].x, start.y + d[1].y}
        diag_coord := Coord{start.x + d[2].x, start.y + d[2].y}
        top := "empty"
        left := "empty"
        diag := "empty"
        if top_coord.x >= 0 && top_coord.x < len(grid[0]) && top_coord.y >= 0 && top_coord.y < len(grid) {
            top = grid[top_coord.y][top_coord.x]
        }
        if left_coord.x >= 0 && left_coord.x < len(grid[0]) && left_coord.y >= 0 && left_coord.y < len(grid) {
            left = grid[left_coord.y][left_coord.x]
        }
        if diag_coord.x >= 0 && diag_coord.x < len(grid[0]) && diag_coord.y >= 0 && diag_coord.y < len(grid) {
            diag = grid[diag_coord.y][diag_coord.x]
        }


        if top == plot_type && left == plot_type && diag != plot_type {
            corners += 1
        }

        if top != plot_type && left != plot_type {
            corners += 1
        }
    }


    return corners
}

func dfs(grid [][]string, start Coord, visited map[Coord]bool, bulk bool) int {

    perimeter := 0
    area := 1

    plot_type := grid[start.y][start.x]

    corners := check_corner(grid, start, plot_type)
    dirs := [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }

    q := []Coord{start}
    visited[start] = true

    for len(q) > 0 {
        pos := q[0]
        q = q[1:]

        for _, d := range dirs {
            newPos := Coord{pos.x + d.x, pos.y + d.y}
            if newPos.x < 0 || newPos.x >= len(grid[0]) || newPos.y < 0 || newPos.y >= len(grid) {
                perimeter += 1
                continue
            }

            if visited[newPos] {
                if grid[newPos.y][newPos.x] != plot_type {
                    perimeter += 1
                }
                continue
            }


            if grid[newPos.y][newPos.x] != plot_type {
                perimeter += 1
            } else {
                visited[newPos] = true
                q = append(q, newPos)
                corners += check_corner(grid, newPos, plot_type)
                area += 1
            }
        }
    }


    if bulk {
        return corners*area
    }
    return perimeter*area
}

func get_prices(grid [][]string, bulk bool) int {
    price := 0

    visited := map[Coord]bool{}

    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if visited[Coord{j, i}] {
                continue
            }
            price += dfs(grid, Coord{j, i}, visited, bulk)
        }
    }


    return price
}

func part1() {
    content := internal.ReadInput("data/data12.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    grid := [][]string{}
    
    for i := 0; i < len(res); i++ {
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            line = append(line, string(org_line[j]))
        }
        grid = append(grid, line)
    }

    total := 0

    total += get_prices(grid, false)

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data12.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    grid := [][]string{}
    
    for i := 0; i < len(res); i++ {
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            line = append(line, string(org_line[j]))
        }
        grid = append(grid, line)
    }

    total := 0

    total += get_prices(grid, true)

    fmt.Println("Part 2: ", total)
}
func main() {
    part1()
    part2()
}
