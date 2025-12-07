import { readFileSync } from "fs";

const data_path = './data7.txt'

const raw_data = readFileSync(data_path).toString().trim()

var data: string[][] = raw_data.split('\n').map((row: string) => row.split(''))


var total_splits = 0

for (let i = 0; i < data.length - 1; i++) {
    const line = data[i]

    for (let j = 0; j < line.length; j++) {
        const char = line[j]

        if (char == '|' || char == 'S') {
            if (data[i + 1][j] == '.') {
                data[i + 1][j] = '|'
            } else if (data[i + 1][j] == '^') {
                data[i + 1][j-1] = '|'
                data[i + 1][j+1] = '|'
                total_splits++
            } }
    }
}

console.log(total_splits)
