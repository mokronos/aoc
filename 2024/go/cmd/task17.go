package main

import (
	"aoc/internal"
	"fmt"
    "strings"
    "strconv"
    "os/exec"
)

func run_program(rega int, regb int, regc int, prog []int, check_valid bool) []int {
    instr_pointer := 0

    output := []int{}
    var lit_op int
    var combo_op int

    for {

        if instr_pointer >= len(prog) {
            break
        }
        opcode := prog[instr_pointer]
        operand := prog[instr_pointer + 1]

        lit_op = operand

        switch operand {
            case 0, 1, 2, 3:
                combo_op = operand
            case 4:
                combo_op = rega
            case 5:
                combo_op = regb
            case 6:
                combo_op = regc
            default:
                panic("Invalid operand")
        }


        switch opcode {
            case 0:
                rega = rega / (1 << combo_op)
            case 1:
                regb = regb ^ lit_op
            case 2:
                regb = combo_op % 8
            case 3:
                if rega != 0 {
                    instr_pointer = lit_op
                    continue
                }
            case 4:
                regb = regb ^ regc
            case 5:
                res := combo_op % 8
                output = append(output, res)
                if check_valid && (len(output) > len(prog) || res != prog[len(output)-1]) {
                        return []int{}
                }
            case 6:
                regb = rega >> combo_op
            case 7:
                regc = rega / (1 << combo_op)
        }

        instr_pointer += 2
    }

    if check_valid {
        if len(output) != len(prog) {
            return []int{}
        }
    }

    return output
}

func rev_program(rega int, regb int, regc int, prog []int) []int {
    output := []int{}

    for rega != 0 {
        // 2,4
        regb = rega % 8
        // 1,3
        regb = regb ^ 3
        // 7,5
        regc = rega >> regb
        // 4,0
        regb = regb ^ regc
        // 1,3
        regb = regb ^ 3
        // 0,3
        rega = rega >> 3
        // 5,5
        output = append(output, regb % 8)

        if (len(output) > len(prog) || output[len(output)-1] != prog[len(output)-1]) {
                return []int{}
        }

        // 3,0
    }

    if len(output) != len(prog) {
        return []int{}
    }

    return output
}


func part1() {
    content := internal.ReadInput("data/data17.txt")
    res := strings.Split(content, "\n")
    rega_str := strings.Split(res[0], ": ")[1]
    rega, _ := strconv.Atoi(rega_str)
    regb_str := strings.Split(res[1], ": ")[1]
    regb, _ := strconv.Atoi(regb_str)
    regc_str := strings.Split(res[2], ": ")[1]
    regc, _ := strconv.Atoi(regc_str)
    prog_str := strings.Split(strings.Split(res[4], ": ")[1], ",")

    prog := []int{}

    for i := 0; i < len(prog_str); i++ {
        num, _ := strconv.Atoi(prog_str[i])
        prog = append(prog, num)
    }

    output := run_program(rega, regb, regc, prog, false)

    output_str := []string{}
    for i := 0; i < len(output); i++ {
        output_str = append(output_str, strconv.Itoa(output[i]))
    }
    output_string := strings.Join(output_str, ",")

    fmt.Println("Part 1: ", output_string)
}

func part2() {
    content := internal.ReadInput("data/data17.txt")
    res := strings.Split(content, "\n")
    // rega_str := strings.Split(res[0], ": ")[1]
    // rega, _ := strconv.Atoi(rega_str)
    regb_str := strings.Split(res[1], ": ")[1]
    regb, _ := strconv.Atoi(regb_str)
    regc_str := strings.Split(res[2], ": ")[1]
    regc, _ := strconv.Atoi(regc_str)
    prog_str := strings.Split(strings.Split(res[4], ": ")[1], ",")

    prog := []int{}

    for i := 0; i < len(prog_str); i++ {
        num, _ := strconv.Atoi(prog_str[i])
        prog = append(prog, num)
    }


    total := 0

    // count := 0
    rega_val := 0

    for {
        // nvm not fast enough
        break
        // count ++
        if rega_val > 0 && rega_val % 10000000 == 0 {
            fmt.Println(rega_val)
        }
        output := rev_program(rega_val, regb, regc, prog)
        if len(output) != 0 {
            total += rega_val
            break
        }
        // if count > 5 {
        //     break
        // }
        rega_val ++
    }

    py_script := "./cmd/task17_2.py"

    cmd := exec.Command("python", py_script)
    out, err := cmd.Output()

    if err != nil {
        println(err.Error())
        return
    }

    fmt.Println("Part 2: ", strings.TrimSpace(string(out)))
}

func main() {
    part1()
    part2()
}
