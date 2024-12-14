package main

import (
	"aoc/internal"
	"fmt"
	"math"
	"regexp"
	"strconv"
	"strings"
)

type Coordf struct {
    x float64
    y float64
}

type Game struct {
    A Coordf
    B Coordf
    Prize Coordf
}

func play(game Game) int {

    a := (game.B.x * game.Prize.y - game.B.y * game.Prize.x) / (game.B.x * game.A.y - game.B.y * game.A.x)
    b := (game.Prize.x - game.A.x * a) / game.B.x


    if math.Round(a) != a || math.Round(b) != b {
        return 0
    }

    return int(a*3 + b)
}

func parse_game(line string) Game {
    re := regexp.MustCompile(`\d+`)
    matches := re.FindAllString(line, -1)

    var game Game
    intax, _ := strconv.Atoi(matches[0])
    game.A.x = float64(intax)
    intay, _ := strconv.Atoi(matches[1])
    game.A.y = float64(intay)
    intbx, _ := strconv.Atoi(matches[2])
    game.B.x = float64(intbx)
    intby, _ := strconv.Atoi(matches[3])
    game.B.y = float64(intby)
    intpx, _ := strconv.Atoi(matches[4])
    game.Prize.x = float64(intpx)
    intpy, _ := strconv.Atoi(matches[5])
    game.Prize.y = float64(intpy)

    return game
}

func part1() {
    content := internal.ReadInput("data/data13.txt")
    lines := strings.Split(content, "\n\n")

    total := 0
    for _, line := range lines {
        if line == "" {
            continue
        }

        game := parse_game(line)

        total += play(game)
    }


    fmt.Println("Part 1: ", total)
}


func part2() {
    content := internal.ReadInput("data/data13.txt")
    lines := strings.Split(content, "\n\n")

    total := 0
    for _, line := range lines {
        if line == "" {
            continue
        }

        adjustment := 10000000000000.0

        game := parse_game(line)
        game.Prize.x += adjustment
        game.Prize.y += adjustment

        total += play(game)
    }


    fmt.Println("Part 2: ", total)
}

func main() {
    part1()
    part2()
}
