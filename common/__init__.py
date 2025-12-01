import sys
from typing import Callable, TypeVar

T = TypeVar('T')
def main(input_fn: Callable[[str], T], solve_fn: Callable[[T], str]) -> None:
    input_str = sys.stdin.read()
    input = input_fn(input_str)
    result = solve_fn(input)
    print(result)
