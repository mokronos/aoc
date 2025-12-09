import { readFileSync } from "fs";

const data_path = './data9.txt'

const raw_data = readFileSync(data_path).toString().trim()

// const raw_data = `7,1
// 11,1
// 11,7
// 9,7
// 9,5
// 2,5
// 2,3
// 7,3`.trim()

const data: string[] = raw_data.split('\n')


function is_invalid(x1: number, y1: number, x2: number, y2: number, data: string[]) {

    const x_min = Math.min(x1, x2)
    const y_min = Math.min(y1, y2)
    const x_max = Math.max(x1, x2)
    const y_max = Math.max(y1, y2)


    for (let i = 0; i < data.length; i++) {
        const j = (i + 1) % data.length
        const [xs, ys] = data[i].split(',').map(Number)
        const [xe, ye] = data[j].split(',').map(Number)

        const lx_min = Math.min(xs, xe)
        const ly_min = Math.min(ys, ye)
        const lx_max = Math.max(xs, xe)
        const ly_max = Math.max(ys, ye)

        // vertical
        if (lx_min == lx_max) {
            if (x_min < lx_min && lx_min < x_max) {
                if ((ly_min <= y_min && ly_max > y_min) || (ly_min < y_max && ly_max >= y_max)) {
                    return true
                }
            }
        }

        // horizontal
        if (ly_min == ly_max) {
            if (y_min < ly_min && ly_min < y_max) {
                if ((lx_min <= x_min && lx_max > x_min) || (lx_min < x_max && lx_max >= x_max)) {
                    return true
                }
            }
        }
    }

    return false
}

let max_area = 0

for (let i = 0; i < data.length - 1; i++) {
    for (let j = i + 1; j < data.length; j++) {

        const [x1, y1] = data[i].split(',').map(Number)
        const [x2, y2] = data[j].split(',').map(Number)

        console.log("Progress: ", i, "/", data.length - 1)

        if (is_invalid(x1, y1, x2, y2, data)) continue

        const area = (Math.abs(x1 - x2) + 1) * (Math.abs(y1 - y2) + 1)
        max_area = Math.max(max_area, area)
    }
}

console.log(max_area)
