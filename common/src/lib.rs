use std::io::Read;

pub mod grid;
pub mod space;

pub fn run<T>(parse_fn: fn(&str) -> T, solve_fn: fn(T) -> i64) {
    let mut input_buf = String::new();
    std::io::stdin()
        .read_to_string(&mut input_buf)
        .expect("Failed to read from stdin");
    let input = parse_fn(&input_buf);
    let result = solve_fn(input);
    println!("{result}");
}
pub fn run_with_args<T, U>(
    parse_args: fn(Vec<String>) -> U,
    parse_fn: fn(&str) -> T,
    solve_fn: fn(U, T) -> i64,
) {
    let args = std::env::args().collect();
    let parsed_args = parse_args(args);
    let mut input_buf = String::new();
    std::io::stdin()
        .read_to_string(&mut input_buf)
        .expect("Failed to read from stdin");
    let input = parse_fn(&input_buf);
    let result = solve_fn(parsed_args, input);
    println!("{result}");
}
