use common::space::vector::Vector3i;

pub type Input = Vec<Vector3i>;

pub fn parse_input(input_str: &str) -> Input {
    input_str
        .lines()
        .map(|line| {
            let mut parts = line.split(",").map(str::parse::<i64>).map(Result::unwrap);
            Vector3i::new(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect()
}

pub fn parse_args(args: Vec<String>) -> usize {
    args[1]
        .parse::<usize>()
        .expect("Failed to parse number of connections")
}
