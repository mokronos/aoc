package main

import (
	"aoc/internal"
	"fmt"
	"slices"
	"sort"
	"strings"
    "encoding/base64"
)

type Gate struct {
    a string
    b string
    op string
    rec string
}

func part1() {
    content := internal.ReadInput("data/data24.txt")
    res := strings.Split(content, "\n\n")

    wires := map[string]bool{}

    for _, wire := range strings.Split(res[0], "\n") {
        parts := strings.Split(wire, ": ")
        wire := parts[0]
        val := parts[1]
        switch val {
            case "0": wires[wire] = false
            case "1": wires[wire] = true
        }
    }

    gates := []Gate{}

    for _, gate := range strings.Split(res[1], "\n") {
        if gate == "" {
            continue
        }
        parts := strings.Split(gate, " ")
        gate := Gate{
            a: parts[0],
            b: parts[2],
            op: parts[1],
            rec: parts[4],
        }
        gates = append(gates, gate)
    }


    todo := []Gate{}
    todo = append(todo, gates...)

    for len(todo) > 0 {
        gate := todo[0]
        todo = todo[1:]
        aval, aok := wires[gate.a]
        bval, bok := wires[gate.b]
        if aok && bok {
            switch gate.op {
                case "AND": wires[gate.rec] = aval && bval
                case "OR": wires[gate.rec] = aval || bval
                case "XOR": wires[gate.rec] = aval != bval
            }
        } else {
            todo = append(todo, gate)
        }
    }


    sorted_z_wires := []string{}
    for wire := range wires {
        if wire[0] == 'z' {
            sorted_z_wires = append(sorted_z_wires, wire)
        }
    }
    slices.Sort(sorted_z_wires)

    rev_bin_num := []string{}

    for _, wire := range sorted_z_wires {
        switch wires[wire] {
            case true: rev_bin_num = append(rev_bin_num, "1")
            case false: rev_bin_num = append(rev_bin_num, "0")
        }
    }

    total := 0

    for i := 0; i < len(rev_bin_num); i++ {
        if rev_bin_num[i] == "1" {
            total += 1 << i
        }
    }

    fmt.Println("Part 1: ", total)
}

func encrypt(password, text string) string {
	encrypted := make([]byte, len(password))
	for i := 0; i < len(password); i++ {
		encrypted[i] = password[i] ^ text[i%len(text)]
	}
	return base64.StdEncoding.EncodeToString(encrypted)
}

func decrypt(encryptedBase64, text string) string {
	encrypted, err := base64.StdEncoding.DecodeString(encryptedBase64)
	if err != nil {
		fmt.Println("Error decoding Base64:", err)
		return ""
	}
	decrypted := make([]byte, len(encrypted))
	for i := 0; i < len(encrypted); i++ {
		decrypted[i] = encrypted[i] ^ text[i%len(text)]
	}
	return string(decrypted)
}

func graph(gates []Gate, gates_map map[string]Gate) {
    for _, gate := range gates {
        if gate.rec[0] == 'z' {
            fmt.Println(gate.rec, "A: ", gate.a, "Op: ", gate.op, "B: ", gate.b)

            fmt.Println("-----------------")
            suba, oka := gates_map[gate.a]
            if oka {
                fmt.Println("SubA: ", suba.rec, "A: ", suba.a, "Op: ", suba.op, "B: ", suba.b)

                fmt.Println("-----")
                subaa, okaa := gates_map[suba.a]
                if okaa {
                    fmt.Println("SubAA: ", subaa.rec, "A: ", subaa.a, "Op: ", subaa.op, "B: ", subaa.b)
                }

                subab, okab := gates_map[suba.b]
                if okab {
                    fmt.Println("SubAB: ", subab.rec, "A: ", subab.a, "Op: ", subab.op, "B: ", subab.b)
                }
            }
            fmt.Println("-----------------")
            subb, okb := gates_map[gate.b]
            if okb {
                fmt.Println("SubB: ", subb.rec, "A: ", subb.a, "Op: ", subb.op, "B: ", subb.b)

                fmt.Println("-----")
                subbb, okbb := gates_map[subb.b]
                if okbb {
                    fmt.Println("SubBB: ", subbb.rec, "A: ", subbb.a, "Op: ", subbb.op, "B: ", subbb.b)
                }

                subba, okba := gates_map[subb.a]
                if okba {
                    fmt.Println("SubBA: ", subba.rec, "A: ", subba.a, "Op: ", subba.op, "B: ", subba.b)
                }
            }

            fmt.Println()
            fmt.Println("*****************")
        }
    }
}

func part2() {
    content := internal.ReadInput("data/data24.txt")
    res := strings.Split(content, "\n\n")

    wires := map[string]bool{}

    for _, wire := range strings.Split(res[0], "\n") {
        parts := strings.Split(wire, ": ")
        wire := parts[0]
        val := parts[1]
        switch val {
            case "0": wires[wire] = false
            case "1": wires[wire] = true
        }
    }

    gates := []Gate{}
    gates_map := map[string]Gate{}

    for _, gate := range strings.Split(res[1], "\n") {
        if gate == "" {
            continue
        }
        parts := strings.Split(gate, " ")
        gate := Gate{
            a: parts[0],
            b: parts[2],
            op: parts[1],
            rec: parts[4],
        }
        gates = append(gates, gate)
        gates_map[gate.rec] = gate
    }

    sorted_x_wires := []string{}
    sorted_y_wires := []string{}
    for wire := range wires {
        if wire[0] == 'x' {
            sorted_x_wires = append(sorted_x_wires, wire)
        }
        if wire[0] == 'y' {
            sorted_y_wires = append(sorted_y_wires, wire)
        }
    }
    slices.Sort(sorted_x_wires)
    slices.Sort(sorted_y_wires)

    bx := byte(0)
    for i, wire := range sorted_x_wires {
        if wires[wire] {
            bx += 1 << i
        }
    }

    by := byte(0)
    for i, wire := range sorted_y_wires {
        if wires[wire] {
            by += 1 << i
        }
    }

    sort.Slice(gates, func(i, j int) bool {
		return gates[i].rec < gates[j].rec
	})

    // uncomment to see explanation/graph (check mistakes against full adders)
    // graph(gates, gates_map)

    encrypted_answer := "G0ZYFkRTaFRYU1EMW3wWHEZcTh1wSQQfQBEJJgICBw=="

    decrypted_answer := decrypt(encrypted_answer, content)

    fmt.Println("Part 2: ", decrypted_answer)
}


func main() {
    part1()
    part2()
}
