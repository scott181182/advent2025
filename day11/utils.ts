import type { Digraph } from "./input";

export function topologicalSort(graph: Digraph): string[] {
    const visited = new Set<string>();
    const result: string[] = [];

    function dfs(node: string) {
        if (visited.has(node)) return;
        visited.add(node);
        for (const neighbor of graph.edges[node] || []) {
            dfs(neighbor);
        }
        result.push(node);
    }

    for (const node of graph.nodes) {
        dfs(node);
    }

    return result.reverse();
}