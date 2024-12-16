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

func print_map(grid [][]string) {
    for i := 0; i < len(grid); i++ {
        fmt.Println(strings.Join(grid[i], ""))
    }
}

func calc_gps(grid [][]string) int {
    total := 0
    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if grid[i][j] == "O" {
                total += i*100 + j
            }
        }
    }
    return total
}

func move(grid [][]string, start Coord, move string) Coord {

    move_map := map[string]Coord{
        "^": Coord{0, -1},
        "v": Coord{0, 1},
        ">": Coord{1, 0},
        "<": Coord{-1, 0},
    }

    move_coord := move_map[move]

    newPos := Coord{start.x + move_coord.x, start.y + move_coord.y}

    if grid[newPos.y][newPos.x] == "#" {
        return start
    }

    if grid[newPos.y][newPos.x] == "O" {
        // push
        tmp_pos := newPos
        for {
            tmp_pos = Coord{tmp_pos.x + move_coord.x, tmp_pos.y + move_coord.y}
            if grid[tmp_pos.y][tmp_pos.x] == "#" {
                return start
            }
            if grid[tmp_pos.y][tmp_pos.x] == "O" {
                continue
            }
            if grid[tmp_pos.y][tmp_pos.x] == "." {
                grid[start.y][start.x] = "."
                grid[tmp_pos.y][tmp_pos.x] = "O"
                grid[newPos.y][newPos.x] = "@"
                return newPos
            }
        }

    }

    grid[start.y][start.x] = "."
    grid[newPos.y][newPos.x] = "@"

    return newPos
}

func part1() {
    content := internal.ReadInput("data/data15.txt")
    res := strings.Split(content, "\n\n")
    grid_str := strings.Split(res[0], "\n")
    moves_str := strings.Split(res[1], "\n")

    grid := [][]string{}

    var start Coord

    for i := 0; i < len(grid_str); i++ {
        if grid_str[i] == "" {
            continue
        }
        line := []string{}
        for j := 0; j < len(grid_str[i]); j++ {
            if grid_str[i][j] == byte('@') {
                start = Coord{j, i}
            }
            line = append(line, string(grid_str[i][j]))
        }
        grid = append(grid, line)
    }


    moves := []string{}
    for i := 0; i < len(moves_str); i++ {
        if moves_str[i] == "" {
            continue
        }
        line_chars := strings.Split(moves_str[i], "")

        for j := 0; j < len(line_chars); j++ {
            moves = append(moves, line_chars[j])
        }
    }

    print_map(grid)
    fmt.Println("Start: ", start)

    for i := 0; i < len(moves); i++ {
        start = move(grid, start, moves[i])
        // fmt.Println("After move: ", i+1)
        // print_map(grid)
    }


    total := 0
    total += calc_gps(grid)


    fmt.Println("Part 1: ", total)
}


func main() {
    part1()
}
