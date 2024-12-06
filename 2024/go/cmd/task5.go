package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "strconv"
)

func indexOf(arr []string, target string) int {
    for i := 0; i < len(arr); i++ {
        if arr[i] == target {
            return i
        }
    }
    return -1
}

func part1() {
    content := internal.ReadInput("data/data5.txt")
    res := strings.Split(content, "\n\n")

    var rules = res[0]
    var lines = res[1]

    var total = 0

    for _, line := range strings.Split(lines, "\n") {
        if line == "" {
            continue
        }
        var pages = strings.Split(line, ",")
        var error = false

        for _, rule := range strings.Split(rules, "\n") {
            if rule == "" {
                continue
            }
            var res = strings.Split(rule, "|")
            var first = res[0]
            var second = res[1]
            var first_index = indexOf(pages, first)
            var second_index = indexOf(pages, second)

            if first_index == -1 || second_index == -1 {
                continue
            }
            if first_index > second_index {
                error = true
                break
            }
        }
        if !error {
            var val, _ = strconv.Atoi(pages[len(pages)/2])
            total += val
        }
    }

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data5.txt")
    res := strings.Split(content, "\n\n")

    var rules = res[0]
    var lines = res[1]

    var total = 0

    for _, line := range strings.Split(lines, "\n") {
        if line == "" {
            continue
        }
        var pages = strings.Split(line, ",")

        var loop = 0

        for {
            var error = false

            for _, rule := range strings.Split(rules, "\n") {
                if rule == "" {
                    continue
                }
                var res = strings.Split(rule, "|")
                var first = res[0]
                var second = res[1]
                var first_index = indexOf(pages, first)
                var second_index = indexOf(pages, second)

                if first_index == -1 || second_index == -1 {
                    continue
                }
                if first_index > second_index {
                    error = true
                    pages[first_index], pages[second_index] = pages[second_index], pages[first_index]
                }
            }
            if !error {
                var val, _ = strconv.Atoi(pages[len(pages)/2])
                if loop != 0 {
                    total += val
                }
                break
            }
            loop++
        }
    }

    fmt.Println("Part 2: ", total)
}


func main() {
    part1()
    part2()
}
