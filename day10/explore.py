import matplotlib.pyplot as plt

from common import main
from day10.input import Machine, parse_input


def explore(machines: list[Machine]):
    fig, axes = plt.subplots(1, 2, figsize=(12, 6))
    # Diagram Size Histogram
    axes[0].hist([len(m.light_diagram) for m in machines])
    axes[0].set_title("Light Diagram Sizes")
    axes[0].set_xlabel("Size")
    axes[0].set_ylabel("Frequency")
    # Schematic Size Histogram
    axes[1].hist([len(m.wiring_schematic) for m in machines])
    axes[1].set_title("Wiring Schematic Sizes")
    axes[1].set_xlabel("Size")
    axes[1].set_ylabel("Frequency")
    plt.tight_layout()
    plt.show()
    return 0


if __name__ == "__main__":
    main(parse_input, explore)
