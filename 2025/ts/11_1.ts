import { readFileSync } from "fs";

const data_path = './data11.txt'

const raw_data = readFileSync(data_path).toString().trim()

const data: string[] = raw_data.split('\n')

const map = new Map<string, string[]>()

for (const line of data) {
    const [key, value] = line.split(": ")
    map.set(key, value.split(" "))
}

const queue: string[] = []

queue.push("you")

var total = 0

while (queue.length > 0) {
    const cur = queue.shift()
    if (cur === undefined) {
        throw new Error("Queue is empty!")
    }
    if (cur === "out") {
        total++
        continue
    }
    for (const next of map.get(cur)!) {
        queue.push(next)
    }
}

console.log(total)
