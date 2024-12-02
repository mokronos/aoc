package main

import (
	"aoc/internal"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func part1() {
    content := internal.ReadInput("data/data1.txt")
    lines := strings.Split(content, "\n")

    var list1 = []int{}
    var list2 = []int{}

    for _, line := range lines {
        if line == "" {
            continue
        }
        var res = strings.Fields(line)
        var first = res[0]
        var second = res[1]

        var firstInt, _ = strconv.Atoi(first)
        var secondInt, _ = strconv.Atoi(second)

        list1 = append(list1, firstInt)
        list2 = append(list2, secondInt)
    }

    slices.Sort(list1)
    slices.Sort(list2)

    var total = 0

    for i := 0; i < len(list1); i++ {

        var diff = list1[i] - list2[i]
        if diff > 0 {
            total += diff
        } else {
            total += diff * -1
        }

    }

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data1.txt")
    lines := strings.Split(content, "\n")

    var list1 = []int{}
    var list2 = []int{}

    for _, line := range lines {
        if line == "" {
            continue
        }
        var res = strings.Fields(line)
        var first = res[0]
        var second = res[1]

        var firstInt, _ = strconv.Atoi(first)
        var secondInt, _ = strconv.Atoi(second)

        list1 = append(list1, firstInt)
        list2 = append(list2, secondInt)
    }

    var total = 0

    for i := 0; i < len(list1); i++ {
        var num = list1[i]

        var amt = 0

        for j := 0; j < len(list2); j++ {

            if num == list2[j] {
                amt++
            }
        }

        total += amt * num
    }

    fmt.Println("Part 2: ", total)
}


func main() {
    part1()
    part2()
}
