import { run } from "../common"
import { BinaryHeap } from "../common/heap";
import { parseInput, type Digraph } from "./input";
import { topologicalSort } from "./utils";

const START_NODE = "svr";
const END_NODE = "out";
const FFT_NODE = "fft";
const DAC_NODE = "dac";

interface NodeCount {
    raw: number;
    fft: number;
    dac: number;
    fft_dac: number;
}

function solve(graph: Digraph): number {
    const nodesInOrder = topologicalSort(graph);
    const pathCounts: Record<string, NodeCount> = Object.fromEntries(nodesInOrder.map(n => [n, { raw: 0, fft: 0, dac: 0, fft_dac: 0 }]));
    pathCounts[START_NODE] = { raw: 1, fft: 0, dac: 0, fft_dac: 0 };

    const nodeStack = new BinaryHeap<string>();
    nodeStack.push(START_NODE, 0);
    while (nodeStack.length > 0) {
        const current = nodeStack.pop()!;

        if (current === FFT_NODE) {
            pathCounts[current]!.fft += pathCounts[current]!.raw;
            pathCounts[current]!.fft_dac += pathCounts[current]!.dac;
        } else if (current === DAC_NODE) {
            pathCounts[current]!.dac += pathCounts[current]!.raw;
            pathCounts[current]!.fft_dac += pathCounts[current]!.fft;
        }

        for (const neighbor of graph.edges[current] || []) {
            pathCounts[neighbor]!.raw += pathCounts[current]!.raw;
            pathCounts[neighbor]!.fft += pathCounts[current]!.fft;
            pathCounts[neighbor]!.dac += pathCounts[current]!.dac;
            pathCounts[neighbor]!.fft_dac += pathCounts[current]!.fft_dac;

            if (!nodeStack.includes(neighbor)) {
                nodeStack.push(neighbor, nodesInOrder.indexOf(neighbor));
            }
        }
    }

    return pathCounts[END_NODE]!.fft_dac;
}

await run(parseInput, solve);
