Grouped = list[list[int]]
Sums = list[int]


def parse_input(inp: str) -> list[list[int]]:
    # Split by blank lines
    return [[int(n) for n in sub.splitlines()]
            for sub in inp.split("\n\n")]


def make_sums(inp: Grouped) -> Sums:
    # O(n + m) = O(m) where n is number of elves and m is number of items
    return [sum(xs) for xs in inp]


def part1(sums: Sums) -> int:
    # O(n) where n is number of elves
    return max(sums)


def part2(sums: Sums) -> int:
    import heapq
    # Make a max-heap and pop 3 times.
    heap = [-n for n in sums]   # Negate to make a max-heap.
    heapq.heapify(heap)
    top_three = (heapq.heappop(heap) for _ in range(3))
    # O(n) where n is number of elves
    return -sum(top_three)


if __name__ == '__main__':
    with open("../day1_input.txt") as f:
        grouped = parse_input(f.read())
        sums = make_sums(grouped)
        print(part1(sums))
        print(part2(sums))
