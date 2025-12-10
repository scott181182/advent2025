from dataclasses import dataclass
import re


@dataclass
class Machine:
    light_diagram: list[bool]
    wiring_schematic: list[list[int]]
    joltage_requirements: list[int]

    @staticmethod
    def parse_line(line: str) -> "Machine":
        diagram_str, rest = line.split(" ", 1)
        schematic_str, joltage_str = rest.rsplit(" ", 1)

        light_diagram = [c == "#" for c in diagram_str[1:-1]]
        joltage_requirements = [int(n) for n in joltage_str[1:-1].split(",")]

        wiring_groups = re.findall(r"\((\d+(,\d+)*)\)", schematic_str)
        wiring_schematic = [
            list(map(int, groups[0].split(","))) for groups in wiring_groups
        ]

        return Machine(light_diagram, wiring_schematic, joltage_requirements)


def parse_input(input_str: str) -> list[Machine]:
    return [Machine.parse_line(line) for line in input_str.splitlines()]
