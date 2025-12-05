from dataclasses import dataclass

@dataclass
class Database:
    fresh_ranges: list[tuple[int, int]]
    available_ingredients: list[int]

def parse_input(input_str: str) -> Database:
    fresh_range_lines, available_ingredient_lines = input_str.split("\n\n")

    return Database(
        fresh_ranges=[
            tuple(map(int, line.split("-"))) for line in fresh_range_lines.splitlines()
        ],
        available_ingredients=list(map(int, available_ingredient_lines.splitlines())),
    )
