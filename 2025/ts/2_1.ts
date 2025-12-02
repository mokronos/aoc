import { readFileSync } from "fs";

const data_path = './data2.txt'

const data = readFileSync(data_path).toString()


function is_invalid(num: number): boolean {
    const num_str = num.toString()
    if (num_str.length % 2 == 1) {
        return false
    }

    const a: string = num_str.slice(0, num_str.length / 2)
    const b: string = num_str.slice(num_str.length / 2)

    return a == b
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
