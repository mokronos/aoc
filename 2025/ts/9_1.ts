import { readFileSync } from "fs";

const data_path = './data9.txt'

const raw_data = readFileSync(data_path).toString().trim()


const data: string[] = raw_data.split('\n')

var max_area = 0

for (let i = 0; i < data.length - 1; i++) {
    for (let j = i + 1; j < data.length; j++) {

        const line_a = data[i]
        const line_b = data[j]

        const [x_a, y_a] = line_a.split(',').map(Number)
        const [x_b, y_b] = line_b.split(',').map(Number)

        const area = (Math.abs(x_a - x_b) + 1) * (Math.abs(y_a - y_b) + 1)
        max_area = Math.max(max_area, area)
    }
}

console.log(max_area)
