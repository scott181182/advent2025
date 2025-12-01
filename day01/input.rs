pub const DIAL_MOD: i64 = 100;
pub const DIAL_START: i64 = 50;

pub type Input = Vec<i64>;

pub fn parse_input(input_str: &str) -> Input {
    input_str.trim().lines().map(|line| {
        let (dir, num) = line.split_at(1);
        let steps: i64 = num.parse().expect("Failed to parse number");
        if dir == "R" { steps } else { -steps }
    }).collect()
}
