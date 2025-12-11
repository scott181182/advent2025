import { run } from "../common"
import { BinaryHeap } from "../common/heap";
import { parseInput, type Digraph } from "./input";
import { topologicalSort } from "./utils";

const START_NODE = "you";
const END_NODE = "out";

function solve(graph: Digraph): number {
    const nodesInOrder = topologicalSort(graph);
    const pathCounts: Record<string, number> = Object.fromEntries(nodesInOrder.map(n => [n, 0]));
    pathCounts[START_NODE] = 1;

    const nodeStack = new BinaryHeap<string>();
    nodeStack.push(START_NODE, 0);
    while (nodeStack.length > 0) {
        const current = nodeStack.pop()!;

        for (const neighbor of graph.edges[current] || []) {
            pathCounts[neighbor]! += pathCounts[current]!;
            if (!nodeStack.includes(neighbor)) {
                nodeStack.push(neighbor, nodesInOrder.indexOf(neighbor));
            }
        }
    }

    return pathCounts[END_NODE]!;
}

await run(parseInput, solve);
