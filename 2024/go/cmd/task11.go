package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "strconv"
)

func blink(stones []int, rounds int) []int {

    for i := 0; i < rounds; i++ {

        fmt.Printf("Progress: %d/%d\n", i, rounds)

        new_stones := []int{}

        for j := 0; j < len(stones); j++ {
            if stones[j] == 0 {
                new_stones = append(new_stones, 1)
            } else if len(strconv.Itoa(stones[j]))%2 == 0 {

                str_num := strconv.Itoa(stones[j])
                first_half := str_num[:len(str_num)/2]
                second_half := str_num[len(str_num)/2:]

                num1, _ := strconv.Atoi(first_half)
                num2, _ := strconv.Atoi(second_half)

                new_stones = append(new_stones, num1, num2)
            } else {
                new_stones = append(new_stones, stones[j]*2024)
            }
        }
        stones = new_stones
    }

    return stones
}


func part1() {
    content := internal.ReadInput("data/data11.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    data := strings.Split(res[0], " ")
    stones := []int{}
    for i := 0; i < len(data); i++ {
        num, _ := strconv.Atoi(data[i])
        stones = append(stones, num)
    }

    rounds := 25

    total := 0
    total += len(blink(stones, rounds))

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data11.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    data := strings.Split(res[0], " ")
    stones := []int{}
    for i := 0; i < len(data); i++ {
        num, _ := strconv.Atoi(data[i])
        stones = append(stones, num)
    }

    // rounds := 75

    zero_lengths := []int{}
    for i := 0; i < 10; i++ {
        zero_lengths = append(zero_lengths, len(blink([]int{0}, i)))
    }

    total := 0

    fmt.Println(zero_lengths)

    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
