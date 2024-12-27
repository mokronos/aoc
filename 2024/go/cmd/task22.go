package main

import (
	"aoc/internal"
	"fmt"
	"math"
	"strconv"
	"strings"
)

func contains_key(keys []string, key string) bool {
    for _, k := range keys {
        if k == key {
            return true
        }
    }
    return false
}


func evolve(num int) int {

    num = num ^ (num * 64) 
    num %= 16777216

    tmp := int(math.Floor(float64(num) / 32))

    num = num ^ tmp
    num %= 16777216

    num = num ^ (num * 2048)
    num %= 16777216

    return num
}

func generate_sequence(secret_number int, amt int) []int {
    seq := []int{secret_number}

    for i := 0; i < amt; i++ {
        init := seq[len(seq) - 1]
        seq = append(seq, evolve(init))
    }

    return seq
}

func part1() {
    content := internal.ReadInput("data/data22.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    secret_numbers := []int{}

    for _, line := range res {
        num, _ := strconv.Atoi(line)
        secret_numbers = append(secret_numbers, num)
    }

    total := 0

    for _, secret_number := range secret_numbers {
        seq := generate_sequence(secret_number, 2000)
        total += seq[len(seq) - 1]
    }

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data22.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    secret_numbers := []int{}

    for _, line := range res {
        num, _ := strconv.Atoi(line)
        secret_numbers = append(secret_numbers, num)
    }

    // secret_numbers = []int{123}
    // amt := 10
    // secret_numbers = []int{1, 2, 3, 2024}
    amt := 2000

    seqs := [][]int{}

    for _, secret_number := range secret_numbers {
        seq := generate_sequence(secret_number, amt)
        seqs = append(seqs, seq)
    }

    price_seqs := [][]int{}

    for _, seq := range seqs {
        price := []int{}
        for i := 0; i < len(seq); i++ {
            num := seq[i]
            num_str := strconv.Itoa(num)
            last_digit_str := num_str[len(num_str) - 1]
            last_digit, _ := strconv.Atoi(string(last_digit_str))
            price = append(price, last_digit)
        }
        price_seqs = append(price_seqs, price)
    }

    changes := [][]int{}

    for _, seq := range price_seqs {
        change := []int{0}
        for i := 0; i < len(seq)-1; i++ {
            change = append(change, seq[i+1] - seq[i])
        }
        changes = append(changes, change)
    }

    // for i:= 0; i < len(changes); i++ {
    //     fmt.Println("Seq: ", i)
    //     for j := 0; j < len(changes[i]); j++ {
    //         fmt.Println(seqs[i][j], ": ", price_seqs[i][j], " (", changes[i][j], ")")
    //     }
    // }

    fmt.Println("Created changes")

    sells := make([]map[string]int, len(changes))
    keys := make(map[string]struct{})


    for i := 0; i < len(changes); i++ {
        sells[i] = make(map[string]int)
        fmt.Println("Progress: ", i, "/", len(changes))
        for j := 1; j < len(changes[i])-3; j++ {

            val := price_seqs[i][j+3]
            key := fmt.Sprintf("%d%d%d%d", changes[i][j], changes[i][j+1], changes[i][j+2], changes[i][j+3])

            if _, ok := keys[key]; !ok {
                keys[key] = struct{}{}
            }

            if _, ok := sells[i][key]; !ok {
                sells[i][key] = val
            }
        }
    }

    fmt.Println("Created sells and keys")

    total := -1
    for key := range keys {
        bananas := 0
        for i := 0; i < len(sells); i++ {
            if _, ok := sells[i][key]; ok {
                bananas += sells[i][key]
            }
        }
        if total == -1 || bananas > total {
            total = bananas
        }
    }


    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
