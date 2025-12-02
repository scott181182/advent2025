from common import main
from day02.input import parse_input

MIN_REPEAT = 2

def explore_patterns(start: int, end: int, pattern_length: int) -> set[int]:
    start_str = str(start)
    start_length = len(start_str)
    end_length = len(str(end))

    pattern_repetition_counts = [
        n // pattern_length
        for n in range(start_length, end_length + 1)
        if n % pattern_length == 0 and n > pattern_length
    ]
    if len(pattern_repetition_counts) == 0:
        return set()
    
    invalid_ids = set()
    
    for pattern_repetitions in pattern_repetition_counts:
        if pattern_repetitions * pattern_length == start_length:
            # Start with starting pattern in `start`.
            candidate_pattern = start_str[:pattern_length]
            candidate_int = int(candidate_pattern)
        else:
            # Start at the beginning for this number of sigfigs.
            candidate_pattern = "1" + ("0" * (pattern_length - 1))
            candidate_int = int(candidate_pattern)

        candidate = int(candidate_pattern * pattern_repetitions)
        while candidate <= end:
            if candidate >= start:
                invalid_ids.add(candidate)

            # Move to next pattern, checking for wraparound.
            candidate_int += 1
            candidate_pattern = str(candidate_int)
            if len(candidate_pattern) > pattern_length:
                # Overflow!
                break

            candidate = int(candidate_pattern * pattern_repetitions)

    return invalid_ids
def find_invalid_ids(start: int, end: int) -> set[int]:
    end_str = str(end)

    # Set comprehension to avoid duplicates.
    return {
        invalid_id
        for pattern_length in range(1, (len(end_str) // MIN_REPEAT) + 1)
        for invalid_id in explore_patterns(start, end, pattern_length)
    }

def solve_part2(id_ranges: list[tuple[int, int]]):
    return sum([
        sum(find_invalid_ids(start, end))
        for start, end in id_ranges
    ])

if __name__ == "__main__":
    main(parse_input, solve_part2)
