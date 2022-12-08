import numpy as np

BitSet = np.ulonglong
Compartments = tuple[BitSet, BitSet]


def parse_compartments(s: str) -> list[Compartments]:
    return [compartments_from_str(line) for line in s.splitlines()]


def compartments_from_str(s: str) -> Compartments:
    first, second = s[:len(s) // 2], s[len(s) // 2:]
    return bitset_from_str(first), bitset_from_str(second)


def bitset_from_str(s: str) -> BitSet:
    n = BitSet(0)
    for c in s:
        n |= BitSet((1 << char_index(c)))
    return n


def char_index(c: str) -> int:
    if ord(c) > ord('a'):
        return ord(c) - ord('a') + 1
    else:
        return ord(c) - ord('A') + 1 + 26


def bitset_value(bs: BitSet) -> int:
    # It's possible many bits are set -> we have to iterate.
    sum_ = 0
    for i in range(64):
        if bs & BitSet(1 << i):
            sum_ += i
    return sum_


def part1(comps: list[Compartments]) -> int:
    return sum(bitset_value(c1 & c2) for c1, c2 in comps)


Group = tuple[BitSet, BitSet, BitSet]


def parse_groups(s: str) -> list[Group]:
    lines = s.splitlines()
    return [(bitset_from_str(lines[i]), bitset_from_str(lines[i + 1]), bitset_from_str(lines[i + 2]))
            for i in range(0, len(lines), 3)]


def group_priority(g: Group) -> int:
    first, second, third = g
    rem = first & second & third
    return ffs(int(rem))


def ffs(n: int) -> int:
    return (n & -n).bit_length() - 1


def part2(groups: list[Group]) -> int:
    return sum(group_priority(g) for g in groups)


if __name__ == '__main__':
    with open("../day3_input.txt") as f:
        s = f.read()
        compartments = parse_compartments(s)
        print(part1(compartments))
        groups = parse_groups(s)
        print(part2(groups))


