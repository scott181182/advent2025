

export interface Digraph {
    /** Set of node identifiers. */
    nodes: Set<string>;
    /** Map from node identifier to list of adjacent node identifiers. */
    edges: Record<string, string[]>;
}

export function parseInput(input: string): Digraph {
    const nodes = new Set<string>();
    const edges: Record<string, string[]> = {};

    for (const line of input.split("\n")) {
        const [from, toList] = line.split(": ");
        if (!from || !toList) {
            throw new Error(`Could not find nodes in line, '${line}'`)
        }

        nodes.add(from);
        edges[from] = toList.split(" ");
        edges[from].forEach(to => nodes.add(to));
    }

    return { nodes, edges };
}