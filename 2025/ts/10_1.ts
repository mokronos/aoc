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

function switch_lights(lights: Lights, wiring: number[]): Lights {
    const new_lights = lights.slice()
    for (let i = 0; i < wiring.length; i++) {
        new_lights[wiring[i]] = !new_lights[wiring[i]]
    }
    return new_lights
}

function solve(lights: Lights, wirings: number[][]): number {
    const queue: Lights[] = []
    const initial_lights = new Array(lights.length).fill(false)
    queue.push(initial_lights)
    queue.push([])

    var count = 1

    while (true) {
        const cur_lights = queue.shift()
        if (cur_lights === undefined) {
            throw new Error("Nothing left in queue!")
        }
        if (cur_lights.length === 0) {
            count++
            queue.push([])
            continue
        }
        for (const wiring of wirings) {
            const new_lights = switch_lights(cur_lights, wiring)
            const same = new_lights.every((x, i) => x === lights[i])
            if (same) {
                return count
            }
            queue.push(new_lights)
        }
    }
}

var total = 0

for (const manual of manuals) {
    const lights = manual.lights
    const wirings = manual.wirings

    const num_presses = solve(lights, wirings)
    total += num_presses
}

console.log(total)
