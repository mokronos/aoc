import { readFileSync } from "fs";

const data_path = './data12.txt'

const raw_data = readFileSync(data_path).toString().trim()

const data: string[] = raw_data.split('\n\n')

type Region = {
    x: number,
    y: number,
    amts: number[]
}

const raw_regions: string[] = data[data.length - 1].split('\n')
const raw_presents: string[] = data.slice(0, data.length - 1)

const presents: string[][] = []

for (var piece of raw_presents) {
    presents.push(piece.split('\n').slice(1))
}

const regions: Region[] = []

for (var region of raw_regions) {
    const [first, second] = region.split(': ')
    const [x, y] = first.split('x').map(Number)
    const amts = second.split(' ').map(Number)
    regions.push({ x, y, amts })
}

function can_fit(region: Region, presents: string[][]): boolean {

    const area = region.x * region.y

    var pot_smallest_area = 0
    for (var i = 0; i < region.amts.length; i++) {
        const present_area = presents[i].map(row => row.split('')).flat().filter(x => x === '#').length
        pot_smallest_area += present_area * region.amts[i]
    }

    var pot_largest_area = 0
    for (var i = 0; i < region.amts.length; i++) {
        const present_area = presents[i].map(row => row.split('')).flat().length
        pot_largest_area += present_area * region.amts[i]
    }

    if (pot_largest_area <= area) {
        return true
    }

    if (pot_smallest_area > area) {
        return false
    }

    throw new Error("Fit can't be easily determined")
}

var total = 0

for (const region of regions) {
    if (can_fit(region, presents)) {
        total++
    }
}

console.log(total)
