


export class BinaryHeap<T> {
    private readonly heap: [T, number][] = [];

    public constructor() { }

    private heapifyUp(index: number): void {
        if (index === 0) return;
        const parentIndex = Math.floor((index - 1) / 2);
        if (this.heap[index]![1] < this.heap[parentIndex]![1]) {
            const tmp = this.heap[index];
            this.heap[index] = this.heap[parentIndex]!;
            this.heap[parentIndex] = tmp!;
            this.heapifyUp(parentIndex);
        }
    }
    private heapifyDown(index: number): void {
        const leftChildIndex = 2 * index + 1;
        const rightChildIndex = 2 * index + 2;
        let smallestIndex = index;

        if (leftChildIndex < this.heap.length && this.heap[leftChildIndex]![1] < this.heap[smallestIndex]![1]) {
            smallestIndex = leftChildIndex;
        }
        if (rightChildIndex < this.heap.length && this.heap[rightChildIndex]![1] < this.heap[smallestIndex]![1]) {
            smallestIndex = rightChildIndex;
        }
        if (smallestIndex !== index) {
            const tmp = this.heap[index];
            this.heap[index] = this.heap[smallestIndex]!;
            this.heap[smallestIndex] = tmp!;
            this.heapifyDown(smallestIndex);
        }
    }

    public get length(): number {
        return this.heap.length;
    }

    public includes(item: T): boolean {
        return this.heap.some(([i, _]) => i === item);
    }
    public push(item: T, priority: number): void {
        this.heap.push([item, priority]);
        this.heapifyUp(this.heap.length - 1);
    }
    public pop(): T | undefined {
        if (this.heap.length === 0) return undefined;
        const root = this.heap[0]![0];
        const last = this.heap.pop()!;
        if (this.heap.length > 0) {
            this.heap[0] = last;
            this.heapifyDown(0);
        }
        return root;
    }
}