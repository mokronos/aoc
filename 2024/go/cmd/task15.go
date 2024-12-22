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

func calc_gps_alt(grid [][]string) int {
    total := 0
    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if grid[i][j] == "[" {
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

func parse_grid(grid_str []string) ([][]string, Coord) {

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
    return grid, start
}

func parse_moves(moves_str []string) []string {
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

    return moves
}

func transform_grid(grid [][]string) ([][]string, Coord) {
    new_grid := [][]string{}
    start := Coord{0, 0}
    for i := 0; i < len(grid); i++ {
        line := []string{}
        for j := 0; j < len(grid[i]); j++ {
            switch grid[i][j] {
                case ".":
                    line = append(line, ".")
                    line = append(line, ".")
                case "#":
                    line = append(line, "#")
                    line = append(line, "#")
                case "@":
                    start.x = j*2
                    start.y = i
                    line = append(line, "@")
                    line = append(line, ".")
                case "O":
                    line = append(line, "[")
                    line = append(line, "]")
            }
        }
        new_grid = append(new_grid, line)
    }
    return new_grid, start
}

func part1() {
    content := internal.ReadInput("data/data15.txt")
    res := strings.Split(content, "\n\n")
    grid_str := strings.Split(res[0], "\n")
    moves_str := strings.Split(res[1], "\n")


    grid, start := parse_grid(grid_str)
    moves := parse_moves(moves_str)

    for i := 0; i < len(moves); i++ {
        start = move(grid, start, moves[i])
        // fmt.Println("After move: ", i+1)
        // print_map(grid)
    }


    total := 0
    total += calc_gps(grid)


    fmt.Println("Part 1: ", total)
}

func create_copy(grid [][]string) [][]string {
    new_grid := [][]string{}
    for i := 0; i < len(grid); i++ {
        line := []string{}
        for j := 0; j < len(grid[i]); j++ {
            line = append(line, grid[i][j])
        }
        new_grid = append(new_grid, line)
    }
    return new_grid
}

func sync_grids(grid [][]string, grid_old [][]string) {
    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[i]); j++ {
            if grid[i][j] != grid_old[i][j] {
                grid[i][j] = grid_old[i][j]
            }
        }
    }
}

func move_alt(grid [][]string, start Coord, move string) Coord {

    move_map := map[string]Coord{
        "^": Coord{0, -1},
        "v": Coord{0, 1},
        ">": Coord{1, 0},
        "<": Coord{-1, 0},
    }

    move_coord := move_map[move]

    new_grid := create_copy(grid)
    cur_pos := start

    q := []Coord{start}
    visited := map[Coord]bool{}
    moved_to := map[Coord]bool{}
    q = append(q, cur_pos)

    for len(q) > 0 {
        cur_pos = q[0]
        q = q[1:]
        if visited[cur_pos] {
            continue
        }
        visited[cur_pos] = true
        new_pos := Coord{cur_pos.x + move_coord.x, cur_pos.y + move_coord.y}

        switch grid[new_pos.y][new_pos.x] {
            case "#":
                return start
            case "[":
                new_grid[new_pos.y][new_pos.x] = grid[cur_pos.y][cur_pos.x]
                moved_to[new_pos] = true
                if !moved_to[cur_pos] {
                    new_grid[cur_pos.y][cur_pos.x] = "."
                }
                q = append(q, new_pos)
                q = append(q, Coord{new_pos.x + 1, new_pos.y})
            case "]":
                new_grid[new_pos.y][new_pos.x] = grid[cur_pos.y][cur_pos.x]
                moved_to[new_pos] = true
                if !moved_to[cur_pos] {
                    new_grid[cur_pos.y][cur_pos.x] = "."
                }
                q = append(q, new_pos)
                q = append(q, Coord{new_pos.x - 1, new_pos.y})
            case ".":
                new_grid[new_pos.y][new_pos.x] = grid[cur_pos.y][cur_pos.x]
                moved_to[new_pos] = true
                if !moved_to[cur_pos] {
                    new_grid[cur_pos.y][cur_pos.x] = "."
                }
        }
    }

    sync_grids(grid, new_grid)
    return Coord{start.x + move_coord.x, start.y + move_coord.y}
}

func part2() {
    content := internal.ReadInput("data/data15.txt")
    res := strings.Split(content, "\n\n")
    grid_str := strings.Split(res[0], "\n")
    moves_str := strings.Split(res[1], "\n")


    grid_old, _ := parse_grid(grid_str)
    moves := parse_moves(moves_str)

    grid, start := transform_grid(grid_old)

    // print_map(grid)

    for i := 0; i < len(moves); i++ {
        start = move_alt(grid, start, moves[i])
        // fmt.Println("After move: ", i+1)
        // print_map(grid)
    }

    // print_map(grid)


    total := 0
    total += calc_gps_alt(grid)


    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
