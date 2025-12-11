import { readFileSync } from "fs";
import { init } from "z3-solver";

const data_path = './data10.txt'

// const raw_data = readFileSync(data_path).toString().trim()

const raw_data = `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`.trim()

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


async function solve(joltage: number[], wirings: number[][]): Promise<number> {
    const matrix: number[][] = []
    for (let i = 0; i < wirings.length; i++) {
        matrix.push(new Array(joltage.length).fill(0))
        for (let j = 0; j < wirings[i].length; j++) {
            matrix[i][wirings[i][j]] = 1
        }
    }

    console.log("matrix")
    for (const row of matrix) {
        console.log(row)
    }
    console.log("joltage", joltage)


    const { Context } = await init();
    const Z3 = Context("main");

    const R = matrix
    const C = joltage

    const m = R.length;
    const n = R[0].length;

    const solver = new Z3.Optimize();

    // decision vars x_0 ... x_7
    const x = Array.from({ length: m }, (_, i) =>
        Z3.Int.const(`x_${i}`)
    );

    // x[i] >= 0
    x.forEach(v => solver.add(v.ge(0)));

    // Column constraints: sum_i x_i * R[i][j] == C[j]
    for (let j = 0; j < n; j++) {
        const terms = [];
        for (let i = 0; i < m; i++) {
            terms.push(x[i].mul(R[i][j]));
        }
        solver.add(Z3.Sum(...terms).eq(C[j]));
    }

    // Minimize total number of operations
    const totalOps = Z3.Sum(...x);
    solver.minimize(totalOps);

    // Solve
    const result = solver.check();
    console.log("Status:", result);

    if (result !== "sat" && result !== "optimal") {
        console.log("No solution found.");
        return 0;
    }

    const model = solver.model();

    const solution = x.map(v => Number(model.eval(v).toString()));
    console.log("x =", solution);
    console.log("sum =", solution.reduce((a, b) => a + b, 0));


    throw new Error("not implemented")
}

var total = 0

for (const manual of manuals) {
    const joltage = manual.joltage
    const wirings = manual.wirings

    const num_presses = await solve(joltage, wirings)
    total += num_presses
}

console.log(total)
