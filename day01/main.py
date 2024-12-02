# https://adventofcode.com/2024/day/1

import collections
import os
from typing import List

def parse_lines(lines: List[str]) -> tuple[List[int], List[int]]:
    left = []
    right = []
    for line in lines:
        l, r = line.split()
        left.append(int(l))
        right.append(int(r))
    return left, right

def part1(lines: List[str], expected: int | None = None) -> int:
    total = 0
    # Start of implementation #
    left, right = parse_lines(lines)
    left.sort()
    right.sort()
    for l, r in zip(left, right):
        total += abs(r - l)
    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def part2(lines: List[str], expected: int | None = None) -> int:
    # Start of implementation #
    left, right = parse_lines(lines)
    counts = collections.defaultdict(lambda: 0)
    for r in right:
        counts[r] += 1
    total = 0
    for l in left:
        total += l * counts[l]
    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def process_file(filename: str, expected_part1: int | None = None, expected_part2: int | None = None):
    path = os.path.join(os.path.dirname(__file__), filename)
    with open(path, "r") as input_file:
        lines = input_file.readlines()

    p1 = part1(lines, expected_part1)
    p2 = part2(lines, expected_part2)
    return p1, p2


if __name__ == "__main__":
    example_part1_expected = 11
    example_part2_expected = 31
    ex1, _ = process_file("example1", example_part1_expected, None)
    _, ex2 = process_file("example1", None, example_part2_expected)
    print("Example:")
    print(f"Part 1 expected {example_part1_expected} got {ex1}: {example_part1_expected == ex1}")
    print(f"Part 2 expected {example_part2_expected} got {ex2}: {example_part2_expected == ex2}")

    p1, p2 = process_file("input")
    print("Input:")
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")