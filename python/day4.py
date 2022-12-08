Range = tuple[int, int]
RangePair = tuple[Range, Range]


def parse_range_pairs(s: str) -> list[RangePair]:
    return [range_pair_from_str(line) for line in s.splitlines()]


def range_pair_from_str(s: str) -> RangePair:
    r1_str, r2_str = s.strip().split(",")
    return range_from_str(r1_str), range_from_str(r2_str)


def range_from_str(s: str) -> Range:
    n1, n2 = s.split("-")
    return int(n1), int(n2)


def range_fully_contains(r1: Range, r2: Range) -> bool:
    """Return true if r1 fully contains r2."""
    min_1, max_1 = r1
    min_2, max_2 = r2
    return min_1 <= min_2 and max_2 <= max_1


def pair_fully_contains(p: RangePair) -> bool:
    r1, r2 = p
    return range_fully_contains(r1, r2) or range_fully_contains(r2, r1)


def part1(pairs: list[RangePair]) -> int:
    return sum(pair_fully_contains(p) for p in pairs)


def range_overlaps(r1: Range, r2: Range) -> bool:
    min_1, max_1 = r1
    min_2, max_2 = r2
    return min_1 <= max_2 and max_1 >= min_2


def pair_overlaps(p: RangePair) -> bool:
    r1, r2 = p
    return range_overlaps(r1, r2)


def part2(pairs: list[RangePair]) -> int:
    return sum(pair_overlaps(p) for p in pairs)


if __name__ == '__main__':
    with open("../day4_input.txt") as f:
        pairs = parse_range_pairs(f.read())
        print(part1(pairs))
        print(part2(pairs))

