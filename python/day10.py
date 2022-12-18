from typing import Iterable, Optional

Instruction = Optional[int]
InstructionStream = Iterable[Instruction]
NOOP_STR = "noop"


def parse_line(line: str) -> Instruction:
    if line == NOOP_STR:
        return None

    _addx, n = line.split(" ")
    assert _addx == "addx"
    return int(n)


def parse_input(s: str) -> list[Instruction]:
    return [parse_line(line) for line in s.splitlines()]


INDICES = [20, 60, 100, 140, 180, 220]


def get_x_values(s: InstructionStream) -> list[int]:
    xs = [1]
    for instr in s:
        xs.append(xs[-1])
        if instr is not None:
            xs.append(xs[-1] + instr)

    return xs


def part1(s: InstructionStream) -> int:
    xs = get_x_values(s)

    return sum(i * xs[i - 1] for i in INDICES)


WIDTH = 40
HEIGHT = 6


def part2(s: InstructionStream) -> list[str]:
    xs = get_x_values(s)

    lines = []
    line = ""
    for i, x in enumerate(xs):
        x = x % WIDTH
        if abs(x - (i % WIDTH)) <= 1:
            line += "#"
        else:
            line += "."
        if i % WIDTH == (WIDTH - 1):
            lines.append(line)
            line = ""

    assert all(len(line) == WIDTH for line in lines)
    return lines


if __name__ == '__main__':
    fn = "../day10_input.txt"
    with open(fn) as f:
        instrs = parse_input(f.read())
        print(part1(instrs))
        lines = part2(instrs)
        print("\n".join(lines))
