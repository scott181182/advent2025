pub type Input = Vec<(usize, usize)>;

pub fn parse_input(input_str: &str) -> Input {
    input_str.trim().split(",").map(|range| {
        let (start, end) = range.split_once("-")
            .expect("Failed to split range");
        let start_num: usize = start.parse().expect("Failed to parse start number");
        let end_num: usize = end.parse().expect("Failed to parse end number");
        (start_num, end_num)
    }).collect()
}
