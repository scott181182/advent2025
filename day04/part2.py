import numpy as np
from scipy.ndimage import convolve

from common import main
from day04.input import parse_input



KERNEL = np.array([
    [1, 1, 1],
    [1, 0, 1],
    [1, 1, 1],
])

def solve_part2(map: np.ndarray):
    total = 0

    while True:
        convolved = convolve(map, KERNEL, mode="constant")
        filtered = ((convolved < 4) & (map > 0)).astype(int)
        remove_count = filtered.sum()
        if remove_count == 0:
            break
        total += remove_count
        map -= filtered

    return total

if __name__ == "__main__":
    main(parse_input, solve_part2)
