import numpy as np
from scipy.ndimage import convolve

from common import main
from day04.input import parse_input



KERNEL = np.array([
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
])

def solve_part1(map: np.ndarray):
    convolved = convolve(map, KERNEL, mode="constant")
    filtered = ((convolved < 4) & (map > 0)).astype(int)
    return filtered.sum()

if __name__ == "__main__":
    main(parse_input, solve_part1)
