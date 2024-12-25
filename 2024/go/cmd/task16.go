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

type Move struct {
    pos Coord
    dir int
}

type State struct {
    pos Coord
    dir int
    score int
}

type State2 struct {
    pos Coord
    dir int
    score int
    coords []Coord
}

func part1() {
    content := internal.ReadInput("data/data16.txt")
    res := strings.Split(content, "\n")

    grid := [][]string{}
    start := Coord{0, 0}

    for i := 0; i < len(res); i++ {
        if res[i] == "" {
            continue
        }
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            if res[i][j] == byte('S') {
                start = Coord{j, i}
            }
            line = append(line, string(org_line[j]))
        }
        grid = append(grid, line)
    }

    dirs := [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }


    dir_change := [3]int{3, 0, 1}

    q := []State{{start, 3, 0}}

    visited := make([][][]int, len(grid))

    for i := 0; i < len(grid); i++ {
        visited[i] = make([][]int, len(grid[i]))
        for j := 0; j < len(grid[i]); j++ {
            visited[i][j] = make([]int, 4)
            for k := 0; k < 4; k++ {
                visited[i][j][k] = -1
            }
        }
    }

    lowest_score := -1

    for len(q) > 0 {
        state := q[0]
        q = q[1:]


        for _, d := range dir_change {

            new_dir := (state.dir + d) % 4
            newPos := Coord{state.pos.x + dirs[new_dir].x, state.pos.y + dirs[new_dir].y}
            new_score := state.score
            if d == 0 {
                new_score += 1
            } else {
                new_score += 1001
            }


            saved_score := visited[newPos.y][newPos.x][new_dir]
            if saved_score != -1{
                if saved_score < new_score {
                    continue
                }
                visited[newPos.y][newPos.x][new_dir] = new_score
            }

            visited[newPos.y][newPos.x][new_dir] = new_score

            if lowest_score != -1 && new_score >= lowest_score {
                continue
            }

            switch grid[newPos.y][newPos.x] {
                case "#":
                    continue
                case "S":
                    continue
                case "E":
                    if lowest_score == -1 || new_score < lowest_score {
                        lowest_score = new_score
                    }
                default:
                    q = append(q, State{newPos, new_dir, new_score})
            }
    }
    }

    total := 0
    total += lowest_score


    fmt.Println("Part 1: ", total)
}

func vis_path(grid [][]string, path []Coord) {
    for i := 0; i < len(grid); i++ {
        new_row := []string{}
        for j := 0; j < len(grid[i]); j++ {
            cont := false
            for k := 0; k < len(path); k++ {
                if (path[k] == Coord{j, i}) {
                    cont = true
                }
            }
            if cont {
                new_row = append(new_row, "O")
            } else {
                new_row = append(new_row, grid[i][j])
            }
        }
        fmt.Println(strings.Join(new_row, ""))
    }
}

func part2() {
    content := internal.ReadInput("data/data16.txt")
    res := strings.Split(content, "\n")

    grid := [][]string{}
    start := Coord{0, 0}

    for i := 0; i < len(res); i++ {
        if res[i] == "" {
            continue
        }
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            if res[i][j] == byte('S') {
                start = Coord{j, i}
            }
            line = append(line, string(org_line[j]))
        }
        grid = append(grid, line)
    }

    dirs := [4]Coord{
        {0, 1},
        {-1, 0},
        {0, -1},
        {1, 0},
    }


    dir_change := [3]int{3, 0, 1}

    q := []State2{{start, 3, 0, []Coord{start}}}

    visited := make([][][]int, len(grid))

    for i := 0; i < len(grid); i++ {
        visited[i] = make([][]int, len(grid[i]))
        for j := 0; j < len(grid[i]); j++ {
            visited[i][j] = make([]int, 4)
            for k := 0; k < 4; k++ {
                visited[i][j][k] = -1
            }
        }
    }

    lowest_score := -1
    finished_paths := []State2{}

    for len(q) > 0 {
        state := q[0]
        q = q[1:]


        for _, d := range dir_change {

            new_dir := (state.dir + d) % 4
            newPos := Coord{state.pos.x + dirs[new_dir].x, state.pos.y + dirs[new_dir].y}
            var new_score int
            if d == 0 {
                new_score = state.score + 1
            } else {
                new_score = state.score + 1001
            }
            new_coords := []Coord{}
            for i := 0; i < len(state.coords); i++ {
                new_coords = append(new_coords, state.coords[i])
            }
            new_coords = append(new_coords, newPos)


            saved_score := visited[newPos.y][newPos.x][new_dir]
            if saved_score != -1{
                if saved_score < new_score {
                    continue
                }
                visited[newPos.y][newPos.x][new_dir] = new_score
            }

            visited[newPos.y][newPos.x][new_dir] = new_score

            if lowest_score != -1 && new_score > lowest_score {
                continue
            }

            switch grid[newPos.y][newPos.x] {
            case "#":
                continue
            case "S":
                continue
            case "E":
                finished_paths = append(finished_paths, State2{newPos, new_dir, new_score, new_coords})
                if lowest_score == -1 || new_score < lowest_score {
                    lowest_score = new_score
                }
            default:
                q = append(q, State2{newPos, new_dir, new_score, new_coords})
            }
        }
    }

    total := 0

    path_set := map[Coord]bool{}

    for _, path := range finished_paths {
        if path.score > lowest_score {
            continue
        }
        for _, coord := range path.coords {
            path_set[coord] = true
        }
    }

    total += len(path_set)

    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
