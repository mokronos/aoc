import { readFileSync } from "fs";

const data_path = './data4.txt'

const data = readFileSync(data_path).toString().trimEnd().split('\n')

const dirs = [-1, 0, 1]

function remove_rolls(data: string[][]): [number, string[][]] {
    var total = 0
    var new_data = data.map(r => [...r])

    for (var y = 0; y < data.length; y++) {
        for (var x = 0; x < data[y].length; x++) {

            if (data[y][x] === '.') continue

            var paper_rolls = 0
            for (const dx of dirs) {
                for (const dy of dirs) {
                    if (dx === 0 && dy === 0) continue
                    const nx = x + dx
                    const ny = y + dy
                    if (nx < 0 || nx >= data[y].length || ny < 0 || ny >= data.length) continue
                    if (data[ny][nx] === '@') paper_rolls++
                }
            }
            if (paper_rolls < 4) {total++; new_data[y][x] = '.'}
        }
    }
    return [total, new_data]
}

var data_ = data.slice()
var total = 0

while (true) {
    var [rolls, data_] = remove_rolls(data_)
    if (rolls === 0) break
    total += rolls
}

console.log(total)
