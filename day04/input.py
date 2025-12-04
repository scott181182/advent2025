import numpy as np

def parse_input(input_str: str) -> np.ndarray:
    return np.array([
        [ int(c == "@") for c in line ]
        for line in input_str.strip().splitlines()
    ])
