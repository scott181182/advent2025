from dataclasses import dataclass
from functools import reduce
from operator import xor

from common import main
from day10.input import Machine, parse_input


@dataclass
class TightMachine:
    light_diagram: int
    wiring_schematic: list[int]

    @staticmethod
    def from_machine(machine: Machine) -> "TightMachine":
        # Convert light diagram to bitmask
        bitmask = 0
        for i, light_on in enumerate(machine.light_diagram):
            if light_on:
                bitmask |= 1 << i
        # Convert wiring schematics to bitmasks
        flat_schematic = [
            sum(1 << i for i in indices) for indices in machine.wiring_schematic
        ]
        return TightMachine(bitmask, flat_schematic)


def solve_part1(machines: list[Machine]):
    machines = [TightMachine.from_machine(m) for m in machines]

    total = 0
    for machine in machines:
        min_combo = len(machine.wiring_schematic)
        for combo in range(0, 1 << len(machine.wiring_schematic)):
            if combo.bit_count() >= min_combo:
                continue
            schematic_masks = [
                machine.wiring_schematic[i]
                for i in range(combo.bit_length())
                if combo & (1 << i) > 0
            ]
            result = reduce(xor, schematic_masks, 0)
            if result == machine.light_diagram:
                min_combo = combo.bit_count()
        total += min_combo
    return total


if __name__ == "__main__":
    main(parse_input, solve_part1)
