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

var num_keypad = [][]string{
    {"7", "8", "9"},
    {"4", "5", "6"},
    {"1", "2", "3"},
    {"#", "0", "A"},
}

var dir_keypad = [][]string{
    {"#", "^", "A"},
    {"<", "v", ">"},
}

func get_coord(c string) Coord {
    for i := 0; i < len(num_keypad); i++ {
        for j := 0; j < len(num_keypad[i]); j++ {
            if num_keypad[i][j] == c {
                return Coord{j, i}
            }
        }
    }
    return Coord{0, 0}
}

func find_num_path(start string, end string) []string {

    start_coord := get_coord(start)
    end_coord := get_coord(end)

    path := []string{}

    if start_coord.x <= end_coord.x {
        if start_coord.y <= end_coord.y {
            x_diff := end_coord.x - start_coord.x
            y_diff := end_coord.y - start_coord.y
            if end_coord.y == 3 && start_coord.x == 0 {
                for i := 0; i < x_diff; i++ {
                    path = append(path, ">")
                }
                for i := 0; i < y_diff; i++ {
                    path = append(path, "v")
                }
            } else {
                for i := 0; i < y_diff; i++ {
                    path = append(path, "v")
                }
                for i := 0; i < x_diff; i++ {
                    path = append(path, ">")
                }
            }
        } else {
            x_diff := end_coord.x - start_coord.x
            y_diff := start_coord.y - end_coord.y
            for i := 0; i < y_diff; i++ {
                path = append(path, "^")
            }
            for i := 0; i < x_diff; i++ {
                path = append(path, ">")
            }
        }
    } else {
        if start_coord.y <= end_coord.y {
            x_diff := start_coord.x - end_coord.x
            y_diff := end_coord.y - start_coord.y
            for i := 0; i < x_diff; i++ {
                path = append(path, "<")
            }
            for i := 0; i < y_diff; i++ {
                path = append(path, "v")
            }
        } else {
            x_diff := start_coord.x - end_coord.x
            y_diff := start_coord.y - end_coord.y
            if start_coord.y == 3 && end_coord.x == 0 {
                for i := 0; i < y_diff; i++ {
                    path = append(path, "^")
                }
                for i := 0; i < x_diff; i++ {
                    path = append(path, "<")
                }
            } else {
                for i := 0; i < x_diff; i++ {
                    path = append(path, "<")
                }
                for i := 0; i < y_diff; i++ {
                    path = append(path, "^")
                }
            }
        }
    }

    return path
}

var memo = map[MemoKey]int{}

func find_dir_path(start string, end string) []string {

    // state := PathState{start, end}
    // if path, exists := dirPathCache[state]; exists {
    //     return path
    // }

    var path []string

    switch start {
        case "A":
            switch end {
                case "A":
                    path = []string{}
                case "^":
                    path = []string{"<"}
                case ">":
                    path = []string{"v"}
                case "v":
                    path = []string{"<", "v"}
                case "<":
                    path = []string{"v", "<", "<"}
        }
        case "^":
            switch end {
                case "A":
                    path = []string{">"}
                case "^":
                    path = []string{}
                case ">":
                    path = []string{"v", ">"}
                case "v":
                    path = []string{"v"}
                case "<":
                    path = []string{"v", "<"}
        }
        case ">":
            switch end {
                case "A":
                    path = []string{"^"}
                case "^":
                    path = []string{"<", "^"}
                case ">":
                    path = []string{}
                case "v":
                    path = []string{"<"}
                case "<":
                    path = []string{"<", "<"}
        }
        case "v":
            switch end {
                case "A":
                    path = []string{"^", ">"}
                case "^":
                    path = []string{"^"}
                case ">":
                    path = []string{">"}
                case "v":
                    path = []string{}
                case "<":
                    path = []string{"<"}
        }
        case "<":
            switch end {
                case "A":
                    path = []string{">", ">", "^"}
                case "^":
                    path = []string{">", "^"}
                case ">":
                    path = []string{">", ">"}
                case "v":
                    path = []string{">"}
                case "<":
                    path = []string{}
        }
    }

    // dirPathCache[state] = path

    return path
}

type PathState struct {
    start string
    end string
}

type MemoKey struct {
    start string
    end string
    depth int
}

func length(start string, end string, depth int) int {

    if depth == 0 {
        return len(find_dir_path(start, end)) + 1
    }

    if memo[MemoKey{start, end, depth}] != 0 {
        return memo[MemoKey{start, end, depth}]
    }

    path := find_dir_path(start, end)


    path = append([]string{"A"}, path...)
    path = append(path, "A")

    complexity := 0
    for i := 0; i < len(path) - 1; i++ {
        start := string(path[i])
        end := string(path[i+1])
        l := length(start, end, depth - 1)
        complexity += l
    }

    memo[MemoKey{start, end, depth}] = complexity

    return complexity
}

func solve(code string, num_dir_keypads int) int {

    path := []string{}

    prepped_code := "A" + code

    for i := 0; i < len(prepped_code) - 1; i++ {
        p := find_num_path(string(prepped_code[i]), string(prepped_code[i+1]))
        p = append(p, "A")
        path = append(path, p...)
    }


    complexity := 0
    path = append([]string{"A"}, path...)
    for i := 0; i < len(path) - 1; i++ {
        start := string(path[i])
        end := string(path[i+1])
        c := length(start, end, num_dir_keypads-1)
        complexity += c
    }

    num_part, _ := strconv.Atoi(code[:len(code)-1])

    // fmt.Println("Code: ", code)
    // fmt.Println("Complexity: ", complexity)
    // fmt.Println("Num: ", num_part)

    return complexity * num_part
}

func part1() {
    content := internal.ReadInput("data/data21.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    codes := res

    total := 0
    for _, code := range codes {
        total += solve(code, 2)
    }


    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data21.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    codes := res

    total := 0
    for _, code := range codes {
        total += solve(code, 25)
    }


    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
