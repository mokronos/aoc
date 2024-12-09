package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "strconv"
)

func create_block_map(disk_map []string) []string {
    block_map:= []string{}

    free := false
    id := 0
    for i := 0; i < len(disk_map); i++ {
        if free {
            block_len, _ := strconv.Atoi(disk_map[i])
            for j := 0; j < block_len; j++ {
                block_map = append(block_map, ".")
            }
            free = false
        } else {
            block_len, _ := strconv.Atoi(disk_map[i])
            for j := 0; j < block_len; j++ {
                block_map = append(block_map, strconv.Itoa(id))
            }
            id++
            free = true
        }
    }
    return block_map
}


func checksum(block_map []string) int {
    total := 0
    for i := 0; i < len(block_map); i++ {
        if block_map[i] == "." {
            continue
        }

        num, _ := strconv.Atoi(block_map[i])

        total += num * i
    }

    return total
}

func part1() {
    content := internal.ReadInput("data/data9.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]
    disk_map := strings.Split(res[0], "")


    block_map := create_block_map(disk_map)


    start := 0

    for i := len(block_map) - 1; i >= 0; i-- {
        num := block_map[i]
        if num == "." {
            continue
        }
        for j := start; j < i; j++ {
            if block_map[j] == "." {
                start = j
                block_map[j] = num
                block_map[i] = "."
                break
            }
        }
    }


    total := 0
    total = checksum(block_map)

    fmt.Println("Part 1: ", total)
}

func part2() {
    content := internal.ReadInput("data/data9.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]
    disk_map := strings.Split(res[0], "")


    block_map := create_block_map(disk_map)


    i := len(block_map) - 1
    for i >= 0 {
        num := block_map[i]
        if num == "." {
            i--
            continue
        } else
        {
            block_length:= 1
            block_num := num
            block_indices := []int{}
            block_indices = append(block_indices, i)
            i--
            for {
                if i < 0 {
                    break
                }
                if block_map[i] == block_num {
                    block_length++
                    block_indices = append(block_indices, i)
                    i--
                } else {
                    break
                }
            }

            for j := 0; j < i; j++ {
                if block_map[j] == "." {
                    enough_space := true
                    for k := 0; k < block_length; k++ {
                        if block_map[j+k] != "." {
                            enough_space = false
                            break
                        }
                    }

                    if enough_space {
                        for k := 0; k < block_length; k++ {
                            block_map[j+k] = block_num
                        }
                        for i := 0; i < len(block_indices); i++ {
                            block_map[block_indices[i]] = "."
                        }
                        break
                    }
                }
            }
        }
    }


    total := 0
    total = checksum(block_map)

    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
