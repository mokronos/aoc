package main

import (
	"aoc/internal"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func part1() {
    content := internal.ReadInput("data/data2.txt")
    lines := strings.Split(content, "\n")

    var total = 0

    for _, line := range lines {
        if line == "" {
            continue
        }
        var res = strings.Fields(line)

        if safe(res, 0) {
            total++
        }
        
    }

    fmt.Println("Part 1: ", total)
}


func part2() {
    content := internal.ReadInput("data/data2.txt")
    lines := strings.Split(content, "\n")

    var total = 0

    for _, line := range lines {
        if line == "" {
            continue
        }
        var res = strings.Fields(line)

        if safe(res, 0) {
            total++
            continue
        }

        for i := 0; i < len(res); i++ {
            var tmp = slices.Clone(res)
            tmp = append(tmp[:i], tmp[i+1:]...)
            if safe(tmp, 0) {
                total++
                break
            }
        }

    }

    fmt.Println("Part 2: ", total)
}


func safe(s []string, allowed_errors int) bool {
    var prev, _ = strconv.Atoi(s[0])
    var second, _ = strconv.Atoi(s[1])
    var errors = 0
    if second > prev {
        for i := 1; i < len(s); i++ {
            var num, _ = strconv.Atoi(s[i])
            if num <= prev || num - 3 > prev {
                errors++
                continue
            }
            prev = num
        }
    } else
    {
        for i := 1; i < len(s); i++ {
            var num, _ = strconv.Atoi(s[i])
            if num >= prev  || num + 3 < prev {
                errors++
                continue
            }
            prev = num
        }
    }
    if errors > allowed_errors {
        return false
    }
    return true
}


func main() {
    part1()
    part2()
}
