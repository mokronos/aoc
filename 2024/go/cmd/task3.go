package main

import (
	"aoc/internal"
	"fmt"
    "regexp"
    "strconv"
)

func part1() {
    content := internal.ReadInput("data/data3.txt")

    var total = 0


    var pattern = `mul\(([0-9]{1,3}),([0-9]{1,3})\)`

    re := regexp.MustCompile(pattern)

    for _, res := range re.FindAllStringSubmatch(content, -1) {
        var first, _ = strconv.Atoi(res[1])
        var second, _ = strconv.Atoi(res[2])
        total += first * second
    }

    fmt.Println("Part 1: ", total)
}


func part2() {
    content := internal.ReadInput("data/data3.txt")

    var total = 0


    var pattern = `mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)`

    re := regexp.MustCompile(pattern)

    var mult = true

    for _, res := range re.FindAllStringSubmatch(content, -1) {

        if res[0] == "don't()" {
            mult = false
            continue
        }

        if res[0] == "do()" {
            mult = true
            continue
        }

        if !mult {
            continue
        }

        var first, _ = strconv.Atoi(res[1])
        var second, _ = strconv.Atoi(res[2])
        total += first * second
    }

    fmt.Println("Part 2: ", total)
}


func main() {
    part1()
    part2()
}
