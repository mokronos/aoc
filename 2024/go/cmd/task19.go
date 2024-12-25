package main

import (
	"aoc/internal"
	"fmt"
    "strings"
)


func check_design(design string, patterns []string, memo map[string]int) int {

    if design == "" {
        return 1
    }

    if memo[design] > 0 {
        return memo[design]
    }

    memo[design] = 0


    for i := range patterns {
        l := len(patterns[i])
        if l > len(design) {
            continue
        }
        beginning := design[:l]
        rest := design[l:]
        if beginning == patterns[i] {
            c := check_design(rest, patterns, memo)
            memo[design] += c
        }
    }

    return memo[design]
}


func part1() {
    content := internal.ReadInput("data/data19.txt")
    res := strings.Split(content, "\n\n")

    patterns := strings.Split(res[0], ", ")

    designs := strings.Split(res[1], "\n")
    designs = designs[:len(designs)-1]

    total := 0

    for i := 0; i < len(designs); i++ {
        memo := map[string]int{}
        if check_design(designs[i], patterns, memo) > 0 {
            total++
        }
    }

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data19.txt")
    res := strings.Split(content, "\n\n")

    patterns := strings.Split(res[0], ", ")

    designs := strings.Split(res[1], "\n")
    designs = designs[:len(designs)-1]

    total := 0

    for i := 0; i < len(designs); i++ {
        memo := map[string]int{}
        count := check_design(designs[i], patterns, memo)
        total += count
    }

    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
