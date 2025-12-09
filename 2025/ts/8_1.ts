import { readFileSync } from "fs";

const data_path = './data8.txt'

const raw_data = readFileSync(data_path).toString().trim()

// const raw_data = `162,817,812
// 57,618,57
// 906,360,560
// 592,479,940
// 352,342,300
// 466,668,158
// 542,29,236
// 431,825,988
// 739,650,466
// 52,470,668
// 216,146,977
// 819,987,18
// 117,168,530
// 805,96,715
// 346,949,466
// 970,615,88
// 941,993,340
// 862,61,35
// 984,92,344
// 425,690,689`.trim()

const NUM_LOOPS = 1000
const NUM_CIRCUITS = 3

const data: string[] = raw_data.split('\n')

type Coord = {
    x: number
    y: number
    z: number
}

type Connection = {
    a: Coord
    b: Coord
}

function add_conn(a: Coord, b: Coord, conns: Coord[][]) {

    var added = false
    for (let i = 0; i < conns.length; i++) {
        const circuit = conns[i]
        if (circuit.includes(a)) {
            if (circuit.includes(b)) {
                return conns
            } else {
                circuit.push(b)
                added = true
                break
            }
        }
        if (circuit.includes(b)) {
            if (circuit.includes(a)) {
                return conns
            } else {
                circuit.push(a)
                added = true
                break
            }
        }
    }

    if (!added) conns.push([a, b])

    var to_combine: number[] = []

    for (let i = 0; i < conns.length; i++) {
        const circuit = conns[i]
        if (circuit.includes(a)) {
            to_combine.push(i)
        }
    }
    if (to_combine.length == 2) {
        const [i, j] = to_combine
        const combined = Array.from(
            new Map(
                [...conns[i], ...conns[j]].map(c => [`${c.x},${c.y},${c.z}`, c])
            ).values()
        )
        conns.splice(j, 1)
        conns[i] = combined
    }

    to_combine = []

    for (let i = 0; i < conns.length; i++) {
        const circuit = conns[i]
        if (circuit.includes(b)) {
            to_combine.push(i)
        }
    }
    if (to_combine.length == 2) {
        const [i, j] = to_combine
        const combined = Array.from(
            new Map(
                [...conns[i], ...conns[j]].map(c => [`${c.x},${c.y},${c.z}`, c])
            ).values()
        )
        conns.splice(j, 1)
        conns[i] = combined
    }
    return conns
}



const dist = (a: Coord, b: Coord) => {
    return Math.sqrt(Math.pow(a.x - b.x, 2) + Math.pow(a.y - b.y, 2) + Math.pow(a.z - b.z, 2))
}

const coords: Coord[] = data.map((row) => {
    const [x, y, z] = row.split(',').map(Number)
    return { x, y, z }
})

const dists = new Map<number, Connection>()

for (let i = 0; i < coords.length; i++) {
    for (let j = i + 1; j < coords.length; j++) {
        dists.set(dist(coords[i], coords[j]), { a: coords[i], b: coords[j] })
    }
}

const sorted_dists = [...dists.entries()].sort((a, b) => a[0] - b[0])

var connections: Coord[][] = []

var loops = 0
for (let i = 0; i < sorted_dists.length; i++) {
    const [_, connection] = sorted_dists[i]
    console.log("*".repeat(100))
    console.log(i)
    console.log(loops)
    console.log(
        "CUR:",
        `[${connection.a.x},${connection.a.y},${connection.a.z}|${connection.b.x},${connection.b.y},${connection.b.z}]`
    )
    console.log(
        "OLD:",
        connections.map(c =>
            `[${c.map(p => `${p.x},${p.y},${p.z}`).join("|")}]`
        ).join(" | ")
    )

    const snapshot = JSON.stringify(connections)
    connections = add_conn(connection.a, connection.b, connections)

    console.log(
        "NEW:",
        connections.map(c =>
            `[${c.map(p => `${p.x},${p.y},${p.z}`).join("|")}]`
        ).join(" | ")
    )

    if (1 || snapshot !== JSON.stringify(connections)) {
        loops++
    }
    if (loops == NUM_LOOPS) {
        break
    }

}


var total = 1
const circuit_lengths = connections.map((circuit) => circuit.length).sort((a, b) => a - b).reverse()

console.log(circuit_lengths)

for (const l of circuit_lengths.slice(0, NUM_CIRCUITS)) {
    total *= l
}


console.log(total)
