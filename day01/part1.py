from common import main
from day01.input import parse_input

DIAL_MOD = 100
DIAL_START = 50

def solve_part1(steps: list[int]):
    dial_position = DIAL_START

    count = 0
    for step in steps:
        dial_position = (dial_position + step) % DIAL_MOD
        if dial_position == 0:
            count += 1

    return count

if __name__ == "__main__":
    main(parse_input, solve_part1)
