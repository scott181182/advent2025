def parse_input(input_str: str) -> list[tuple[int, int]]:
    return [
        (int(id_range.split("-")[0]), int(id_range.split("-")[1]))
        for id_range in input_str.split(",")
    ]
