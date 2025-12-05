import { readFileSync } from "fs";

const data_path = './data5.txt'

const data = readFileSync(data_path).toString().trimEnd()

const ranges = data.split('\n\n')[0].split('\n')

const ranges_sorted = ranges.sort((a: string, b: string) => parseInt(a.split('-')[0]) - parseInt(b.split('-')[0]))

class Range {
    start: number
    end: number
    constructor(start: number, end: number) {
        this.start = start
        this.end = end
    }
}

const ranges_cleaned: Range[] = []

ranges_cleaned.push(new Range(parseInt(ranges_sorted[0].split('-')[0]), parseInt(ranges_sorted[0].split('-')[1])))

for (const range of ranges.slice(1)) {
    const [start, end] = range.split('-').map((x: string) => parseInt(x))

    const latest_range = ranges_cleaned[ranges_cleaned.length - 1]

    if (start <= latest_range.end && end >= latest_range.end) {
        ranges_cleaned[ranges_cleaned.length - 1].end = end
    }

    if (start > latest_range.end) {
        ranges_cleaned.push(new Range(start, end))
    }
}


var total = 0

for (const range of ranges_cleaned) {
    total += range.end - range.start + 1
}

console.log(total)
