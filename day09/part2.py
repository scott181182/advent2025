import bisect
from dataclasses import dataclass
from itertools import chain


from common import main
from day09.input import Point, parse_input


@dataclass
class RectilinearPolygon:
    """
    A polygon made exclusively of horizontal and vertical lines.
    """

    # List of vertical line tuples of the form (x, y0, y1).
    vertical_lines: list[tuple[int, int, int]]
    # List of horizontal line tuples of the form (y, x0, x1).
    horizontal_lines: list[tuple[int, int, int]]

    @staticmethod
    def from_points(points: list[Point]):
        vertical_lines = []
        horizontal_lines = []
        for p0, p1 in chain(zip(points[:-1], points[1:]), [(points[-1], points[0])]):
            if p0.x == p1.x:
                x0 = min(p0.y, p1.y)
                x1 = max(p0.y, p1.y)
                vertical_lines.append((p0.x, x0, x1))
            elif p0.y == p1.y:
                x0 = min(p0.x, p1.x)
                x1 = max(p0.x, p1.x)
                horizontal_lines.append((p0.y, x0, x1))

        vertical_lines.sort(key=lambda tup: tup[0])
        horizontal_lines.sort(key=lambda tup: tup[0])

        return RectilinearPolygon(vertical_lines, horizontal_lines)

    def intersects_rectangle(
        self, top: int, right: int, bottom: int, left: int
    ) -> bool:
        # Check vertical lines intersections
        start_idx = bisect.bisect_right(
            self.vertical_lines, left, key=lambda tup: tup[0]
        )
        end_idx = bisect.bisect_left(self.vertical_lines, right, key=lambda tup: tup[0])
        for i in range(start_idx, end_idx):
            _, x0, x1 = self.vertical_lines[i]
            if not ((x0 >= top and x1 >= top) or (x0 <= bottom and x1 <= bottom)):
                return True

        # Check horizontal lines intersections
        start_idx = bisect.bisect_right(
            self.horizontal_lines, bottom, key=lambda tup: tup[0]
        )
        end_idx = bisect.bisect_left(self.horizontal_lines, top, key=lambda tup: tup[0])
        for i in range(start_idx, end_idx):
            _, x0, x1 = self.horizontal_lines[i]
            if not ((x0 >= right and x1 >= right) or (x0 <= left and x1 <= left)):
                return True

        # No intersections found!
        return False


def calculate_area(p0: Point, p1: Point) -> int:
    return (abs(p1.x - p0.x) + 1) * (abs(p1.y - p0.y) + 1)


def solve_part2(points: list[Point]) -> int:
    polygon = RectilinearPolygon.from_points(points)

    max_area = 0
    for i, p0 in enumerate(points):
        for j in range(i + 1, len(points)):
            p1 = points[j]

            new_area = calculate_area(p0, p1)
            if new_area <= max_area:
                # Not worth checking.
                continue

            # Check if it's a valid region!
            top = max(p0.y, p1.y)
            bottom = min(p0.y, p1.y)
            left = min(p0.x, p1.x)
            right = max(p0.x, p1.x)

            is_valid = not polygon.intersects_rectangle(top, right, bottom, left)
            if is_valid:
                max_area = new_area

    return int(max_area)


if __name__ == "__main__":
    main(parse_input, solve_part2)
