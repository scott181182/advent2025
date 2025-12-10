from dataclasses import dataclass

import numpy as np
from scipy.optimize import milp, LinearConstraint

from common import main
from day10.input import Machine, parse_input


@dataclass
class TightMachine:
    # Basis (inverse) transformation matrix.
    wiring_schematic: np.ndarray
    # Target vector.
    joltage_requirements: np.ndarray

    @staticmethod
    def from_machine(machine: Machine) -> "TightMachine":
        transform = np.zeros(
            (len(machine.wiring_schematic), len(machine.joltage_requirements)),
            dtype=float,
        )
        for i, indices in enumerate(machine.wiring_schematic):
            for idx in indices:
                transform[i, idx] = 1
        return TightMachine(
            transform, (np.array(machine.joltage_requirements)).transpose()
        )


def solve_part2(machines: list[Machine]):
    machines = [TightMachine.from_machine(m) for m in machines]

    total = 0
    for machine in machines:
        res = milp(
            np.ones(machine.wiring_schematic.shape[0]),
            integrality=np.ones(machine.wiring_schematic.shape[0]),
            constraints=LinearConstraint(
                machine.wiring_schematic.T,
                lb=machine.joltage_requirements,
                ub=machine.joltage_requirements,
            ),
        )
        if not res.success:
            raise ValueError(f"Could not find solution for {machine}")
        if res.status != 0:
            raise ValueError(f"Could not find optimal solution for {machine}")

        total += int(res.fun)
    return total


if __name__ == "__main__":
    main(parse_input, solve_part2)
