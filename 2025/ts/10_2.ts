import { readFileSync } from "fs";
import { init } from "z3-solver";

// Initialize Z3 once globally (much more stable than initializing in a loop)
const { Context } = await init();
const Z3 = Context("main");

const raw_data = `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}`.trim();

const data: string[] = raw_data.split('\n');

type Manual = {
    lights: boolean[]
    wirings: number[][]
    joltage: number[]
}

const manuals: Manual[] = [];

// Parse Data
for (let i = 0; i < data.length; i++) {
    const entry = data[i].split(" ");
    const first = entry[0];
    const middle = entry.slice(1, entry.length - 1);
    const last = entry[entry.length - 1];

    const lights = first.slice(1, first.length - 1).split("").map(x => x === "#");
    const wirings: number[][] = [];
    for (let j = 0; j < middle.length; j++) {
        const row = middle[j].slice(1, middle[j].length - 1).split(",").map(Number);
        wirings.push(row);
    }
    const joltage = last.slice(1, last.length - 1).split(",").map(Number);

    manuals.push({ lights, wirings, joltage });
}

async function solve(joltage: number[], wirings: number[][]): Promise<number> {
    const matrix: number[][] = [];
    for (let i = 0; i < wirings.length; i++) {
        matrix.push(new Array(joltage.length).fill(0));
        for (let j = 0; j < wirings[i].length; j++) {
            matrix[i][wirings[i][j]] = 1;
        }
    }

    const R = matrix;
    const C = joltage;
    const m = R.length;
    const n = R[0].length;

    const solver = new Z3.Optimize();

    // 1. Use Z3.Int (global Context), not solver.Int
    const x = Array.from({ length: m }, (_, i) =>
        Z3.Int.const(`x_${i}`)
    );

    // x[i] >= 0
    x.forEach(v => solver.add(v.ge(0)));

    // Column constraints
    for (let j = 0; j < n; j++) {
        const terms = [];
        for (let i = 0; i < m; i++) {
            if (R[i][j] !== 0) {
                // Optimization: Don't multiply by 1
                if (R[i][j] === 1) {
                    terms.push(x[i]);
                } else {
                    terms.push(x[i].mul(R[i][j]));
                }
            }
        }
        
        // Handle cases where a column has no inputs
        if (terms.length > 0) {
            solver.add(Z3.Sum(...terms).eq(C[j]));
        } else if (C[j] !== 0) {
            // If sum is empty but joltage required > 0, it's impossible
            solver.add(Z3.Bool.val(false)); 
        }
    }

    // Minimize total number of operations
    const totalOps = Z3.Sum(...x);
    solver.minimize(totalOps);

    // 2. CRITICAL: await the check()
    const result = await solver.check();

    if (result !== "sat" && result !== "optimal") {
        console.log("No solution found.");
        return 0;
    }

    const model = solver.model();
    const solution = x.map(v => Number(model.eval(v).toString()));
    const sum = solution.reduce((a, b) => a + b, 0);

    return sum;
}

var total = 0;

for (const manual of manuals) {
    const num_presses = await solve(manual.joltage, manual.wirings);
    console.log(`Manual solved: ${num_presses} presses`);
    total += num_presses;
}

console.log("Total:", total);
