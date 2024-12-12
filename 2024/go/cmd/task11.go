package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "strconv"
)

func get(m map[int]int, key int) int {
    val, ok := m[key]
    if !ok {
        return 0
    }
    return val
}

func blink(stones []int, rounds int) int {

    counter:= make(map[int]int)

    for i := 0; i < len(stones); i++ {
        counter[stones[i]] = get(counter, stones[i]) + 1
    }

    for i := 0; i < rounds; i++ {

        fmt.Printf("Progress: %d/%d\n", i, rounds)

        new_counter := make(map[int]int)

        for val, count := range counter {
            if val == 0 {
                new_counter[1] = get(new_counter, 1) + count
            } else if len(strconv.Itoa(val))%2 == 0 {

                str_num := strconv.Itoa(val)
                first_half := str_num[:len(str_num)/2]
                second_half := str_num[len(str_num)/2:]

                num1, _ := strconv.Atoi(first_half)
                num2, _ := strconv.Atoi(second_half)

                new_counter[num1] = get(new_counter, num1) + count
                new_counter[num2] = get(new_counter, num2) + count

            } else {
                new_counter[val*2024] = get(new_counter, val*2024) + count
            }
        }
        counter = new_counter
    }

    count := 0

    for _, val := range counter {
        count += val
    }

    return count
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
    total += blink(stones, rounds)

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

    rounds := 75

    total := 0
    total += blink(stones, rounds)


    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
