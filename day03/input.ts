export function parseInput(input: string): number[][] {
    return input.split("\n")
        .map(line => line.split("").map(c => parseInt(c, 10)));
}
