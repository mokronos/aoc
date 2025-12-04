import { readFileSync } from "fs";

const data_path = './data3.txt'

const data = readFileSync(data_path).toString()

var total = 0

for (const bank of data.split('\n')) {

    if (!bank) continue

    var max_a = Number(bank[0])
    var max_a_idx = 0
    for (var i = 1; i < bank.length - 1; i++) {

        const num = Number(bank[i])
        if (num > max_a) {
            max_a = num
            max_a_idx = i
        }
    }

    var max_b = Number(bank[max_a_idx + 1])

    for (var i = max_a_idx + 1; i < bank.length; i++) {

        const num = Number(bank[i])
        max_b = Math.max(max_b, num)
    }

    const jolts = Number(String(max_a) + String(max_b))

    total += jolts

}

console.log(total)
