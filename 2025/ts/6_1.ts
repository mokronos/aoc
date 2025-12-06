import { readFileSync } from "fs";

const data_path = './data6.txt'

const data = readFileSync(data_path).toString().trim().split('\n')

// const data = `123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   +  `.trimEnd().split('\n')


const whitespace = /\s+/

const lines: string[][] = []

for (const line of data.slice(0, data.length - 1)) {
    const vals = line.trim().split(whitespace)
    lines.push(vals)
}

const ops = data[data.length - 1].split(whitespace)

var total = 0

for (var i = 0; i < ops.length; i++) {
    const op = ops[i]
    var col_total = 0

    if (op === '+') {
        for (const line of lines) {
            const val = line[i]

            col_total += parseInt(val)
        }
    }

    if (op === '*') {
        col_total++
        for (const line of lines) {
            const val = line[i]

            col_total *= parseInt(val)
        }
    }
    total += col_total
}

console.log(total)
