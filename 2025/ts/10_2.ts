import { readFileSync } from "fs";

const data_path = './data10.txt'

const raw_data = readFileSync(data_path).toString().trim()

// const raw_data = `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`.trim()

const data: string[] = raw_data.split('\n')

type Lights = boolean[]

type Manual = {
    lights: Lights
    wirings: number[][]
    joltage: number[]
}

const manuals: Manual[] = []

for (let i = 0; i < data.length; i++) {
    const entry = data[i].split(" ")
    const first = entry[0]
    const middle = entry.slice(1, entry.length - 1)
    const last = entry[entry.length - 1]

    const lights = first.slice(1, first.length - 1).split("").map(x => x === "#")
    const wirings: number[][] = []
    for (let j = 0; j < middle.length; j++) {
        const row = middle[j].slice(1, middle[j].length - 1).split(",").map(Number)
        wirings.push(row)
    }
    const joltage = last.slice(1, last.length - 1).split(",").map(Number)

    manuals.push({
        lights,
        wirings: wirings,
        joltage
    })
}

function add_joltage(machine: number[], wiring: number[]): number[] {
    const new_lights = machine.slice()
    for (const pos of wiring) {
        new_lights[pos] += 1
    }
    return new_lights
}

function not_possible(machine: number[], joltage: number[]): boolean {
    for (let i = 0; i < machine.length; i++) {
        if (machine[i] > joltage[i]) {
            return true
        }
    }
    return false
}

function solve(joltage: number[], wirings: number[][]): number {
    const queue: number[][] = []
    const initial_machine = new Array(joltage.length).fill(0)
    queue.push(initial_machine)
    queue.push([])

    var count = 1

    while (true) {
        const cur_machine = queue.shift()
        if (cur_machine === undefined) {
            throw new Error("Nothing left in queue!")
        }
        if (cur_machine.length === 0) {
            count++
            queue.push([])
            console.log("Updated count to", count)
            continue
        }

        if (not_possible(cur_machine, joltage)) {
            continue
        }

        for (const wiring of wirings) {
            const new_machine = add_joltage(cur_machine, wiring)
            const same = new_machine.every((x, i) => x === joltage[i])
            if (same) {
                return count
            }
            queue.push(new_machine)
        }
    }
}

var total = 0

for (const manual of manuals) {
    const joltage = manual.joltage
    const wirings = manual.wirings

    const num_presses = solve(joltage, wirings)
    total += num_presses
}

console.log(total)
