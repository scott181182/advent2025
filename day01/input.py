def parse_input(input_str: str) -> list[int]:
    instructions = []
    for line in input_str.strip().splitlines():
        direction = line[0]
        steps = int(line[1:])
        instructions.append(steps if direction == 'R' else -steps)
    return instructions
