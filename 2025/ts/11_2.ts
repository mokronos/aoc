import { readFileSync } from "fs";
const data_path = './data11.txt'
const raw_data = readFileSync(data_path).toString().trim()

// const raw_data = `svr: aaa bbb
// aaa: fft
// fft: ccc
// bbb: tty
// tty: ccc
// ccc: ddd eee
// ddd: hub
// hub: fff
// eee: dac
// dac: fff
// fff: ggg hhh
// ggg: out
// hhh: out`.trim()

const data: string[] = raw_data.split('\n')
const map = new Map<string, string[]>()

for (const line of data) {
    const [key, value] = line.split(": ")
    map.set(key, value.split(" "))
}

const memo = new Map<string, number>()

const ways = (node: string, fftVisited: boolean, dacVisited: boolean): number => {
    if (node === "out") {
        const newFFT = fftVisited || node === "fft"
        const newDAC = dacVisited || node === "dac"
        return newFFT && newDAC ? 1 : 0
    }

    const key = `${node}:${fftVisited ? 1 : 0}:${dacVisited ? 1 : 0}`
    if (memo.has(key)) return memo.get(key)!

    const newFFT = fftVisited || node === "fft"
    const newDAC = dacVisited || node === "dac"

    let total = 0
    for (const next of map.get(node)!) {
        total += ways(next, newFFT, newDAC)
    }

    memo.set(key, total)
    return total
}

const total = ways("svr", false, false)
console.log(total)
