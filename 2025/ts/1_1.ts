import { readFileSync } from "fs";

const data_path = './data1.txt'

const data = readFileSync(data_path).toString().split('\n')

Number.prototype.mod = function (n) {
    return ((this % n) + n) % n;
};

const max = 100
var cur = 50
var zeros = 0

for (let i = 0; i < data.length; i++) {

    const line = data[i]
    const [a, b] = [line[0], line.slice(1)]

    const b_num = parseInt(b)

    if (a === 'L') {
        cur -= b_num
        cur = cur.mod(max)
    }

    if (a === 'R') {
        cur += b_num
        cur = cur.mod(max)
    }

    if (cur == 0) {
        zeros += 1
    }

}

console.log(zeros)
