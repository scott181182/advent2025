from common import main
from day09.input import Point, parse_input


def calculate_area(p0: Point, p1: Point) -> int:
    return (abs(p1.x - p0.x) + 1) * (abs(p1.y - p0.y) + 1)


def solve_part1(points: list[Point]):
    max_area = 0
    for i, p0 in enumerate(points):
        for j in range(len(points) - 1, i, -1):
            p1 = points[j]
            max_area = max(max_area, calculate_area(p0, p1))

    return max_area


if __name__ == "__main__":
    main(parse_input, solve_part1)
