package main

import (
	"aoc/internal"
    "fmt"
	"strconv"
	"strings"
)

func calc_possible(res int, nums []int, allow_combine bool) bool {

    if len(nums) == 1 {
        return nums[0] == res
    }

    results := []int{nums[0]}

    for i := 1; i < len(nums); i++ {
        var new_results = []int{}
        for j := 0; j < len(results); j++ {

            if allow_combine {
                if combine(results[j], nums[i]) <= res {
                    new_results = append(new_results, combine(results[j], nums[i]))
                }
            }

            if results[j] + nums[i] <= res {
                new_results = append(new_results, results[j] + nums[i])
            }

            if results[j] * nums[i] <= res {
                new_results = append(new_results, results[j] * nums[i])
            }
        }
        results = new_results
    }

    for i := 0; i < len(results); i++ {
        if results[i] == res {
            return true
        }
    }
    return false
}

func part1() {
    content := internal.ReadInput("data/data7.txt")
    lines := strings.Split(content, "\n")

    var total = 0

    for _, line := range lines {
        if line == "" {
            continue
        }
        var tmp = strings.Split(line, ": ")
        var res, _ = strconv.Atoi(tmp[0])
        var nums = []int{}

        for _, numStr := range strings.Split(tmp[1], " ") {
            num, _ := strconv.Atoi(numStr)
            nums = append(nums, num)
        }

        if calc_possible(res, nums, false) {
            total += res
        }
    }

    fmt.Println("Part 1: ", total)
}

func combine(a int, b int) int {
    astr := strconv.Itoa(a)
    bstr := strconv.Itoa(b)

    combstr := astr + bstr

    comb, _ := strconv.Atoi(combstr)
    return comb
}

func part2() {
    content := internal.ReadInput("data/data7.txt")
    lines := strings.Split(content, "\n")

    var total = 0

    for _, line := range lines {
        if line == "" {
            continue
        }
        var tmp = strings.Split(line, ": ")
        var res, _ = strconv.Atoi(tmp[0])
        var nums = []int{}

        for _, numStr := range strings.Split(tmp[1], " ") {
            num, _ := strconv.Atoi(numStr)
            nums = append(nums, num)
        }

        if calc_possible(res, nums, true) {
            total += res
        }

    }

    fmt.Println("Part 2: ", total)
}


func main() {
    part1()
    part2()
}
