import { readFileSync } from "fs";

const data_path = './data2.txt'

const data = readFileSync(data_path).toString()


function is_invalid(num: number): boolean {
    const num_str = num.toString()

    const mid: number = Math.floor(num_str.length / 2)
    const len: number = num_str.length

    for (let i = 1; i <= mid; i++) {
        if (len % i != 0) continue

        const target: string = num_str.slice(0, i)

        var invalid = true

        for (let j = i; j < len; j += i) {
            const str: string = num_str.slice(j, j + i)
            if (str != target) invalid = false
        }

        if (invalid) {
            return true
        }
    }

    return false
}

const ranges = data.split(',')
var sum = 0

for (const range of ranges) {

    const [start, end] = range.split('-')
    const start_num = parseInt(start)
    const end_num = parseInt(end)

    for (let i = start_num; i <= end_num; i++) {
        if (is_invalid(i)) {
            sum += i
        }
    }
}

console.log(sum)
