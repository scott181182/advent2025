from common import main
from day01.input import parse_input

DIAL_MOD = 100
DIAL_START = 50

def rotate_counting_zeros(a: int, b: int, m: int):
    result = (a + b) % m
    passes = abs((a + b) // m)

    if a == 0 and b < 0:
        # We started at zero, so don't count this pass.
        passes -= 1
    if result == 0 and b < 0:
        # Count subtractions that land exactly on zero.
        passes += 1

    return result, passes

def solve_part2(steps: list[int]):
    dial_position = DIAL_START
    
    count = 0
    for step in steps:
        dial_position, passes = rotate_counting_zeros(dial_position, step, DIAL_MOD)
        count += passes

    return count

if __name__ == "__main__":
    main(parse_input, solve_part2)
