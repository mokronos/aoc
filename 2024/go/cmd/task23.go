package main

import (
	"aoc/internal"
	"fmt"
	"slices"
	"strings"
)

func part1() {
    content := internal.ReadInput("data/data23.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]
    total := 0

    pairs := make([][]string, len(res))

    for i, line := range res {
        pairs[i] = make([]string, 2)
        pairs[i][0] = line[0:2]
        pairs[i][1] = line[3:5]
    }

    // find triples
    visited := map[string]bool{}

    triples := [][]string{}
    for i := 0; i < len(pairs); i++ {
        for j := i + 1; j < len(pairs); j++ {

            var triple []string

            if pairs[i][0] == pairs[j][0] {
                triple = []string{pairs[i][1], pairs[i][0], pairs[j][1]}
            } else if pairs[i][0] == pairs[j][1] {
                triple = []string{pairs[i][1], pairs[i][0], pairs[j][0]}
            } else if pairs[i][1] == pairs[j][0] {
                triple = []string{pairs[i][0], pairs[i][1], pairs[j][1]}
            } else if pairs[i][1] == pairs[j][1] {
                triple = []string{pairs[i][0], pairs[i][1], pairs[j][0]}
            }
            if len(triple) != 3 {
                continue
            }
            sorted_triple := []string{triple[0], triple[1], triple[2]}
            slices.Sort(sorted_triple)
            hash := strings.Join(sorted_triple, "")
            if visited[hash] {
                continue
            }
            visited[hash] = true
            triples = append(triples, triple)
        }
    }

    valid_triples := [][]string{}

    for i := 0; i < len(triples); i++ {
        a := triples[i][0]
        b := triples[i][2]

        for pair := range pairs {
            if pairs[pair][0] == a && pairs[pair][1] == b || pairs[pair][0] == b && pairs[pair][1] == a {
                valid_triples = append(valid_triples, triples[i])
                break
            }
        }
    }

    for i := 0; i < len(valid_triples); i++ {
        for j := 0; j < len(valid_triples[0]); j++ {
            if valid_triples[i][j][0] == byte('t') {
                total++
                break
            }
        }
    }


    fmt.Println("Part 1: ", total)
}

func connected(new_pc string, pcs []string, conns map[string][]string) bool {
    for _, pc := range pcs {
        check2 := false
        for _, conn := range conns[pc] {
            if conn == new_pc {
                check2 = true
                break
            }
        }
        if !check2 {
            return false
        }
    }
    return true
}

func part2() {
    content := internal.ReadInput("data/data23.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]

    conns := map[string][]string{}

    for _, line := range res {
        if conns[line[0:2]] != nil {
            conns[line[0:2]] = append(conns[line[0:2]], line[3:5])
        } else {
            conns[line[0:2]] = []string{line[3:5]}
        }
        if conns[line[3:5]] != nil {
            conns[line[3:5]] = append(conns[line[3:5]], line[0:2])
        } else {
            conns[line[3:5]] = []string{line[0:2]}
        }
    }


    best_pcs := []string{}

    for conn := range conns {

        q := []string{conn}
        pcs := []string{conn}

        for len(q) > 0 {
            pos := q[0]
            q = q[1:]

            for _, c := range conns[pos] {
                if connected(c, pcs, conns) {
                    pcs = append(pcs, c)
                    q = append(q, c)
                }
            }
        }

        if len(pcs) > len(best_pcs) {
            best_pcs = pcs
        }
    }

    slices.Sort(best_pcs)

    fmt.Println("Part 2: ", strings.Join(best_pcs, ","))
}

func main() {
    part1()
    part2()
}
