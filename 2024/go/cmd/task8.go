package main

import (
	"aoc/internal"
	"fmt"
    "strings"
)

type Coord struct {
    x int
    y int
}

func getAntis(a Coord, b Coord) []Coord {

    diffx := a.x - b.x
    diffy := a.y - b.y

    antis := []Coord{}
    antis = append(antis, Coord{a.x + diffx, a.y + diffy})
    antis = append(antis, Coord{b.x - diffx, b.y - diffy})

    return antis
}

type Bounds struct {
    minx int
    maxx int
    miny int
    maxy int
}

func getAntisHarm(a Coord, b Coord, bounds Bounds) []Coord {

    diffx := a.x - b.x
    diffy := a.y - b.y



    antis := []Coord{}

    posx := a.x
    posy := a.y

    for posx < bounds.maxx && posy < bounds.maxy && posx >= bounds.minx && posy >= bounds.miny {
        antis = append(antis, Coord{posx, posy})
        posx += diffx
        posy += diffy
    }

    posx = b.x
    posy = b.y

    for posx < bounds.maxx && posy < bounds.maxy && posx >= bounds.minx && posy >= bounds.miny {
        antis = append(antis, Coord{posx, posy})
        posx -= diffx
        posy -= diffy
    }

    return antis
}

func contains_coord(r []Coord, c Coord) bool {
    for _, v := range r {
        if v.x == c.x && v.y == c.y {
            return true
        }
    }
    return false
}

func get_antennas(res []string) map[string][]Coord {
    var antennas = map[string][]Coord{}

    for i, line := range res {
        if line == "" {
            continue
        }

        var res = strings.Split(line, "")

        for j, c := range res {
            if c != "." {
                if _, ok := antennas[c]; ok {
                    antennas[c] = append(antennas[c], Coord{j, i})
                } else {
                    antennas[c] = []Coord{Coord{j, i}}
                }
            }
        }
    }

    return antennas
}

func visualize_grid(res []string, totalantis []Coord) {

    grid := [][]string{}

    for i := 0; i < len(res); i++ {
        line := []string{}
        org_line := res[i]
        for j := 0; j < len(res[i]); j++ {
            line = append(line, string(org_line[j]))
            if contains_coord(totalantis, Coord{j, i}) && line[j] == "." {
                line[j] = "#"
            }
        }
        if len(line) == 0 {
            continue
        }
        grid = append(grid, line)
    }

    for i := 0; i < len(grid); i++ {
        fmt.Println(strings.Join(grid[i], ""))
    }

}

func part1() {
    content := internal.ReadInput("data/data8.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]


    var total = 0

    antennas := get_antennas(res)

    var totalantis = []Coord{}

    for _, v := range antennas {
        for i := 0; i < len(v); i++ {
            for j := i + 1; j < len(v); j++ {
                var a = v[i]
                var b = v[j]

                var antis = getAntis(a, b)

                for i := range antis {
                    anti := antis[i]
                    if anti.x >= 0 && anti.x < len(res[0]) && anti.y >= 0 && anti.y < len(res) {
                        if !contains_coord(totalantis, anti) {
                            totalantis = append(totalantis, anti)
                        }
                }
            }
        }
    }
    }

    total = len(totalantis)

    visualize_grid(res, totalantis)

    fmt.Println("Part 1: ", total)
}

func part2() {

    content := internal.ReadInput("data/data8.txt")
    res := strings.Split(content, "\n")
    res = res[:len(res)-1]


    var total = 0

    antennas := get_antennas(res)

    var totalantis = []Coord{}

    bounds := Bounds{0, len(res[0]), 0, len(res)}

    for _, v := range antennas {
        for i := 0; i < len(v); i++ {
            for j := i + 1; j < len(v); j++ {
                var a = v[i]
                var b = v[j]

                var antis = getAntisHarm(a, b, bounds)

                for i := range antis {
                    anti := antis[i]
                    if anti.x >= 0 && anti.x < len(res[0]) && anti.y >= 0 && anti.y < len(res) {
                        if !contains_coord(totalantis, anti) {
                            totalantis = append(totalantis, anti)
                        }
                }
            }
        }
    }
    }

    total = len(totalantis)

    visualize_grid(res, totalantis)

    fmt.Println("Part 2: ", total)

}

func main() {
    part1()
    part2()
}
