import { run } from "../common"
import { parseInput } from "./input";

function solveLine(digits: number[]): number {
    const leading = digits.slice(0, -1).sort().reverse()[0]!;
    const leadingIdx = digits.indexOf(leading);
    const trailing = digits.slice(leadingIdx + 1).sort().reverse()[0]!;
    return parseInt(`${leading}${trailing}`, 10);
}
function solve(lines: number[][]): number {
    return lines
        .map(solveLine)
        .reduce((acc, val) => acc + val, 0);
}

await run(parseInput, solve);
