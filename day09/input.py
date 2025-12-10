from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int

    def __sub__(self, other: "Point") -> "Point":
        return Point(self.x - other.x, self.y - other.y)


def parse_input(input_str: str) -> list[Point]:
    return [Point(*map(int, line.split(","))) for line in input_str.splitlines()]
