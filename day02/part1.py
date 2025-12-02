from common import main
from day02.input import parse_input


def find_invalid_ids(start: int, end: int) -> list[int]:
    start_str = str(start)

    if len(start_str) & 1 == 0:
        candidate_half = int(start_str[:len(start_str) // 2])
    else:
        # Just an optimization to cut out the odd lengths
        candidate_half = 10 ** (len(start_str) // 2)
    invalid_ids = []

    candidate = int(str(candidate_half) + str(candidate_half))
    while candidate <= end:
        if candidate >= start:
            invalid_ids.append(candidate)

        candidate_half += 1
        candidate = int(str(candidate_half) + str(candidate_half))

    return invalid_ids

def solve_part1(id_ranges: list[tuple[int, int]]):
    invalid_ids = [
        invalid_id
        for range in id_ranges
        for invalid_id in find_invalid_ids(range[0], range[1])
    ]

    return sum(invalid_ids)

if __name__ == "__main__":
    main(parse_input, solve_part1)
