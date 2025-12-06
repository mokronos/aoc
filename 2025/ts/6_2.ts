import { readFileSync } from "fs";

const data_path = './data6.txt'

const data = readFileSync(data_path).toString().split('\n')

const grid = data.slice(0, data.length - 1)

// const grid = `123 328  51 64 
//  45 64  387 23 
//   6 98  215 314
// *   +   *   +  `.split('\n')

function get_subtotal(cur_op: string, cur_vals: number[]) {
    var sub_total = 0
    if (cur_op == '+') {
        for (const val of cur_vals) {
            sub_total += val
        }
    }

    if (cur_op == '*') {
        sub_total++
        for (const val of cur_vals) {
            sub_total *= val
        }
    }

    return sub_total
}


const width = grid[0].length
const height = grid.length

const ops = ['+', '*']

var total = 0
var cur_op: string = ''
var cur_vals: number[] = []
var cur_val: string[] = []
var problem_done = true

for (var x = 0; x < width; x++) {
    problem_done = true
    for (var y = 0; y < height; y++) {
        const val = grid[y][x]
        if (val == ' ') {
            continue
        }
        problem_done = false

        if (ops.includes(val)) {
            cur_op = val
            continue
        }

        cur_val.push(val)
    }
    if (problem_done) {

        const sub_total = get_subtotal(cur_op, cur_vals)

        total += sub_total
        cur_vals = []
        continue
    }
    cur_vals.push(parseInt(cur_val.join('')))
    cur_val = []
}

const sub_total = get_subtotal(cur_op, cur_vals)
total += sub_total

console.log(total)
