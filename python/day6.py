import collections
from typing import Optional


def window_of_unique_chars(s: str, window_size: int) -> Optional[int]:
    # First, read the first window_size chars into a queue. If the window is unique, return window_size.
    window = collections.Counter(s[:window_size])
    q = collections.deque(s[:window_size])
    if len(window) == window_size:
        return window_size

    for i, c in enumerate(s[window_size:]):
        # Evict the least recently added character.
        lrc = q.popleft()
        window[lrc] -= 1
        if not window[lrc]:
            window.pop(lrc)

        # Add the new character.
        q.append(c)
        window[c] += 1

        # If the window is all unique, we're done!
        if len(window) == window_size:
            return i + window_size + 1

    return None


def part1(s: str) -> Optional[int]:
    return window_of_unique_chars(s, window_size=4)


def part2(s: str) -> Optional[int]:
    return window_of_unique_chars(s, window_size=14)


if __name__ == '__main__':
    with open("../day6_input.txt") as f:
        s = f.read()
        print(part1(s))
        print(part2(s))
