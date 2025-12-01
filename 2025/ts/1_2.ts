import { readFileSync } from "fs";

const data_path = './data1.txt'

const data = readFileSync(data_path).toString().split('\n')


Number.prototype.mod = function (n) {
    return ((this % n) + n) % n;
};

const max = 100
var cur = 50
var old = 50
var zeros = 0

for (const line of data) {

    if (!line) break;

    const [a, b] = [line[0], line.slice(1)]

    const to_move = parseInt(b)


    console.log("--------------")
    console.log(a)

    if (a === 'L') {
        old = cur
        cur -= to_move
        cur = cur.mod(max)

        console.log("old", old)
        console.log("cur", cur)
        console.log("to_move", to_move)

        const turns = Math.floor(to_move / max)
        console.log("full_turns", turns)
        zeros += turns
        if (((old < cur) || (cur == 0)) && (old != 0)) {
            zeros += 1
            console.log("partial turn: True")
        }
    }

    if (a === 'R') {
        old = cur
        cur += to_move
        cur = cur.mod(max)

        console.log("old", old)
        console.log("cur", cur)
        console.log("to_move", to_move)

        const turns = Math.floor(to_move / max)
        console.log("full_turns", turns)
        zeros += turns
        if (((old > cur) || (cur == 0)) && (old != 0)) {
            zeros += 1
            console.log("partial turn: True")
        }
    }

}

console.log("\n\n*************")
console.log("password:", zeros)
