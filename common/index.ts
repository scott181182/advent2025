export async function run<T>(
    inputFn: (inputStr: string) => T,
    solveFn: (input: T) => string | number,
) {
    const inputStr = await Bun.stdin.text();
    const input = inputFn(inputStr);
    const result = solveFn(input);
    console.log(result);
}
