import { run } from "../common"
import { parseInput } from "./input";

const BATTERY_COUNT = 12;

function solveLine(digits: number[]): number {
    const chosenBatteries: number[] = [];
    for(let i = BATTERY_COUNT - 1; i >= 0; i--) {
        const next = digits.slice(0, digits.length - i)
            .sort()
            .reverse()
            .at(0)!;
        const nextIdx = digits.indexOf(next);
        digits = digits.slice(nextIdx + 1);
        chosenBatteries.push(next);
    }
    return parseInt(chosenBatteries.join(""), 10);
}
function solve(lines: number[][]): number {
    return lines
        .map(solveLine)
        .reduce((acc, val) => acc + val, 0);
}

await run(parseInput, solve);
