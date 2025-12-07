import { readFileSync } from "fs";

const data_path = './data7.txt'

const raw_data = readFileSync(data_path).toString().trim()

var data: string[][] = raw_data.split('\n').map((row: string) => row.split(''))

var total_timelines = 0
data[0][data[0].indexOf('S')] = '1'

for (let i = 0; i < data.length - 1; i++) {
    const line = data[i]

    for (let j = 0; j < line.length; j++) {
        const char = line[j]

        if (char != '.' && char != '^') {
            const cur_timelines = parseInt(char)

            if (data[i + 1][j] == '.') {
                data[i + 1][j] = char
            } else if (data[i + 1][j] == '^') {
                const left_val = data[i + 1][j-1]

                if (left_val == '.') {
                    data[i + 1][j-1] = char
                } else {
                    data[i + 1][j-1] = (cur_timelines + parseInt(left_val)).toString()
                }

                const right_val = data[i + 1][j+1]

                if (right_val == '.') {
                    data[i + 1][j+1] = char
                } else {
                    data[i + 1][j+1] = (cur_timelines + parseInt(right_val)).toString()
                }

            } else {
                const other_timelines = parseInt(data[i + 1][j])
                data[i + 1][j] = (cur_timelines + other_timelines).toString()
            }
        }
    }
}

data[data.length - 1].forEach((char, _) => { if (char != '.' && char != '^') { total_timelines += parseInt(char) } })

console.log(total_timelines)
