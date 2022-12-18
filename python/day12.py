from collections import deque
from typing import Optional

Cell = tuple[int, int]
Graph = dict[Cell, set[Cell]]
Grid = list[list[int]]


def int_of_char(c: str) -> int:
    assert len(c) == 1
    if c == "S":
        return 0
    elif c == "E":
        return 25
    else:
        return ord(c) - ord("a")


def parse_grid(s: str) -> (Grid, Cell, Cell):
    grid = []
    start_cell = None
    end_cell = None
    for line_i, line in enumerate(s.splitlines()):
        grid_line = []
        for i, c in enumerate(line):
            if c == "S":
                grid_line.append(0)
                start_cell = (line_i, i)
            elif c == "E":
                grid_line.append(25)
                end_cell = (line_i, i)
            else:
                grid_line.append(ord(c) - ord('a'))

        grid.append(grid_line)

    return grid, start_cell, end_cell


def reachable_neighbors(g: Grid, r: int, c: int) -> set[Cell]:
    width = len(g[0])
    height = len(g)
    neighbors = []
    for dr, dc in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        if 0 <= (r + dr) < height and 0 <= (c + dc) < width:
            neighbors.append((r + dr, c + dc))

    elevation = g[r][c]
    reachable = set()
    for nr, nc in neighbors:
        n_elevation = g[nr][nc]
        if n_elevation - elevation <= 1:
            reachable.add((nr, nc))

    return reachable


def graph_of_grid(grid: Grid) -> Graph:
    graph = {}
    for row_i, row in enumerate(grid):
        for col_i, n in enumerate(row):
            graph[(row_i, col_i)] = reachable_neighbors(grid, row_i, col_i)

    return graph


def bfs(g: Graph, start: Cell, end: Cell) -> Optional[int]:
    seen = set()
    q = deque([(start, 1)])
    while q:
        n, length = q.popleft()
        for adj in g[n]:
            if adj in seen:
                continue

            if adj == end:
                return length

            q.append((adj, length + 1))
            seen.add(adj)


part1 = bfs


def unwrap_or(o: Optional[int], default: int) -> int:
    if o is None:
        return default

    return o


def part2(g: Graph, starts: list[Cell], end: Cell) -> int:
    max_len = len(g)
    return min(unwrap_or(bfs(g, s, end), max_len + 1) for s in starts)


if __name__ == '__main__':
    fn = "../day12_input.txt"
    with open(fn) as f:
        grid, s, e = parse_grid(f.read())
        valid_starts = [(r, c) for r, row in enumerate(grid) for c, n in enumerate(row) if n == 0]
        graph = graph_of_grid(grid)
        print(part1(graph, s, e))
        print(part2(graph, valid_starts, e))
