package main

import (
	"aoc/internal"
	"fmt"
	"strings"
)

func part1() {
    content := internal.ReadInput("data/data25.txt")
    res := strings.Split(content, "\n\n")

    locks := [][]int{}
    keys := [][]int{}

    for _, chunk := range res {
        lines := strings.Split(chunk, "\n")
        if lines[0][0] == byte('.') {
            key := []int{}
            for j := 0; j < len(lines[0]); j++ {
                for i := 0; i < len(lines); i++ {
                    if lines[i][j] == byte('#') {
                        key = append(key, len(lines)-i-1)
                        break
                    }
                }
            }
            keys = append(keys, key)
        } else {
            lock := []int{}
            for j := 0; j < len(lines[0]); j++ {
                for i := 0; i < len(lines); i++ {
                    if lines[i][j] == byte('.') {
                        lock = append(lock, i-1)
                        break
                    }
                }
            }
            locks = append(locks, lock)
        }
    }

    total := 0

    for _, key := range keys {
        for _, lock := range locks {
            fits := true
            for c := 0; c < len(key); c++ {
                if key[c] + lock[c] > 5 {
                    fits = false
                    break
                }
            }

            if fits {
                total++
            }
        }
    }

    fmt.Println("Part 1: ", total)
}

func main() {
    part1()
}
