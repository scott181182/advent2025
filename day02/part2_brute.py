import re

from common import main
from day02.input import parse_input

def find_invalid_ids(start: int, end: int) -> set[int]:
    invalid_ids = set()
    for n in range(start, end + 1):
        candidate = str(n)
        for pattern_length in range(1, len(candidate) // 2 + 1):
            if len(candidate) % pattern_length != 0:
                continue
            if re.fullmatch(rf"(\d{{{pattern_length}}})\1+", candidate):
                invalid_ids.add(n)
                break
    return invalid_ids

def solve_part2(id_ranges: list[tuple[int, int]]):
    return sum([
        sum(find_invalid_ids(start, end))
        for start, end in id_ranges
    ])

if __name__ == "__main__":
    main(parse_input, solve_part2)
