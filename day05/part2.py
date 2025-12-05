from functools import reduce

from common import main
from day05.input import Database, parse_input



def range_overlaps(a: tuple[int, int], b: tuple[int, int]) -> bool:
    return not (a[1] < b[0] or b[1] < a[0])
def merge_range(a: tuple[int, int], b: tuple[int, int]) -> tuple[int, int]:
    return (min(a[0], b[0]), max(a[1], b[1]))

class MultiRange:
    def __init__(self):
        self.ranges: list[tuple[int, int]] = []
    def clone(self) -> "MultiRange":
        new_range = MultiRange()
        new_range.ranges = self.ranges.copy()
        return new_range

    def __or__(self, other: tuple[int, int]) -> "MultiRange":
        new_range = self.clone()
        
        overlapping_range = None
        while (overlapping_range := next((r for r in new_range.ranges if range_overlaps(r, other)), None)) is not None:
            other = merge_range(overlapping_range, other)
            new_range.ranges.remove(overlapping_range)
        new_range.ranges.append(other)
        return new_range
    
    def size(self) -> int:
        return sum(end - start + 1 for start, end in self.ranges)
    
def solve_part2(db: Database):
    fresh_ingredients = reduce(
        lambda acc, val: acc | val,
        db.fresh_ranges,
        MultiRange()
    )
    return fresh_ingredients.size()

if __name__ == "__main__":
    main(parse_input, solve_part2)
