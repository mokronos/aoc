import { readFileSync } from "fs";

const data_path = './data3.txt'

const data = readFileSync(data_path).toString()

var total = 0

function get_jolts(bank: string, num_batteries: number = 2): number {

    var start = 0
    var batteries: string[] = []

    for (var i = 0; i < num_batteries; i++) {

        var max_a = Number(bank[start])
        var max_a_idx = start

        for (var j = start; j < bank.length - (num_batteries - i - 1); j++) {

            const num = Number(bank[j])
            if (num > max_a) {
                max_a = num
                max_a_idx = j
            }
        }

        start = max_a_idx + 1
        batteries.push(String(max_a))
    }

    return Number(batteries.join(''))
}


for (const bank of data.split('\n')) {

    if (!bank) continue

    total += get_jolts(bank, 12)
}

console.log(total)
