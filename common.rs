
pub fn run<T>(parse_fn: fn(&str) -> T, solve_fn: fn(T) -> i64) {
    let mut input_buf = String::new();
    std::io::stdin().read_to_string(&mut input_buf)
        .expect("Failed to read from stdin");
    let input = parse_fn(&input_buf);
    let result = solve_fn(input);
    println!("{}", result);
}
