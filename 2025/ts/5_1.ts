import { readFileSync } from "fs";

const data_path = './data5.txt'

const data = readFileSync(data_path).toString().trimEnd()

const ranges = data.split('\n\n')[0].split('\n')
const ingredients = data.split('\n\n')[1].split('\n')

var total = 0

for (const ingredient of ingredients) {
    const id = parseInt(ingredient)

    for (const range of ranges) {
        const [start, end] = range.split('-').map((x: string) => parseInt(x))
        if (id >= start && id <= end) {
            total++
            break
        }
    }
}

console.log(total)
