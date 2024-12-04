package main

import (
	"aoc/internal"
	"fmt"
    "strings"
)

func reverse(r []rune) []rune {
    var res []rune
    for i := len(r) - 1; i >= 0; i-- {
        res = append(res, r[i])
    }
    return res
}

func contains(r []rune, c []rune) int {
    if !strings.Contains(string(r), string(c)) {
        return 0
    }

    var count = 0

    for start := 0; start <= len(r)-len(c); start++ {
        if string(r[start:start+len(c)]) == string(c) {
            count++
        }
    }
    return count
}

func part1() {
    content := internal.ReadInput("data/data4.txt")
    lines := strings.Split(content, "\n")

    var grid [][]rune

    for _, line := range lines {
        if line == "" {
            continue
        }
        var res = []rune(line)
        grid = append(grid, res)
    }

    var total = 0

    var target = []rune("XMAS")

    for i := 0; i < len(grid); i++ {
        var line []rune
        for j := 0; j < len(grid[i]); j++ {
            line = append(line, grid[i][j])
        }
        total += contains(line, target)
        total += contains(reverse(line), target)
    }


    for i := 0; i < len(grid[0]); i++ {
        var line []rune
        for j := 0; j < len(grid); j++ {
            line = append(line, grid[j][i])
        }
        total += contains(line, target)
        total += contains(reverse(line), target)
    }


    var startx = 0
    // diagonals
    for starty:=len(grid)-1; starty >= 0; starty-- {
        var line []rune
        var posx = startx
        var posy = starty
        for posx < len(grid[0]) && posy < len(grid) {
            line = append(line, grid[posy][posx])
            posx++
            posy++
        }
        total += contains(line, target)
        total += contains(reverse(line), target)
    }

    var starty = 0
    for startx:=1; startx < len(grid[0]); startx++ {
        var line []rune
        var posx = startx
        var posy = starty
        for posx < len(grid[0]) && posy < len(grid) {
            line = append(line, grid[posy][posx])
            posx++
            posy++
        }
        total += contains(line, target)
        total += contains(reverse(line), target)
    }

    starty = 0
    for startx:=0; startx < len(grid[0]); startx++ {
        var line []rune
        var posx = startx
        var posy = starty
        for posx >= 0 && posy < len(grid) {
            line = append(line, grid[posy][posx])
            posx--
            posy++
        }
        total += contains(line, target)
        total += contains(reverse(line), target)
    }

    startx = len(grid[0])-1
    for starty:=1; starty < len(grid); starty++ {
        var line []rune
        var posx = startx
        var posy = starty
        for posx >= 0 && posy < len(grid) {
            line = append(line, grid[posy][posx])
            posx--
            posy++
        }
        total += contains(line, target)
        total += contains(reverse(line), target)
    }
    fmt.Println("Part 1: ", total)
}

func square_valid(square [][]rune, target []rune) bool {

    var first = []rune{square[0][0], square[1][1], square[2][2]}
    var second = []rune{square[0][2], square[1][1], square[2][0]}

    if (string(first) == string(target) || string(first) == string(reverse(target))) && (string(second) == string(target) || string(second) == string(reverse(target))) {
        return true
    }

    return false
}

func part2() {
    content := internal.ReadInput("data/data4.txt")
    lines := strings.Split(content, "\n")

    var grid [][]rune

    for _, line := range lines {
        if line == "" {
            continue
        }
        var res = []rune(line)
        grid = append(grid, res)
    }
    var target = []rune("MAS")

    var total = 0

    for i := 1; i < len(grid)-1; i++ {
        for j := 1; j < len(grid[i])-1; j++ {
            var square [][]rune
            square = append(square, grid[i-1][j-1:j+2])
            square = append(square, grid[i][j-1:j+2])
            square = append(square, grid[i+1][j-1:j+2])
            if square_valid(square, target) {
                total += 1
            }
        }
    }

    fmt.Println("Part 2: ", total)
}
        

func main() {
    part1()
    part2()
}
