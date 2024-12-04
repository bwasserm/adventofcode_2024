# https://adventofcode.com/2023/day/1

import os
from typing import List

def part1(lines: List[str], expected: int | None = None) -> int:
    total = 0
    # Start of implementation #
    for line in lines:
        pass
    # End of implementation #
    if expected is not None:
        print(f"Part 1: Expected: {expected} Calculated: {total} Equal: {expected == total}")
    return total


def part2(lines: List[str], expected: int | None = None) -> int:
    total = 0
    # Start of implementation #
    for line in lines:
        pass
    # End of implementation #
    if expected is not None:
        print(f"Part 2: Expected: {expected} Calculated: {total} Equal: {expected == total}")
    return total


def process_file(filename: str, expected_part1: int | None = None, expected_part2: int | None = None):
    path = os.path.join(os.path.dirname(__file__), filename)
    with open(path, "r") as input_file:
        lines = input_file.readlines()

    p1 = part1(lines, expected_part1)
    p2 = part2(lines, expected_part2)
    return p1, p2


if __name__ == "__main__":
    example_part1_expected = 0
    example_part2_expected = 0
    ex1, ex2 = process_file("example", example_part1_expected, example_part2_expected)
    print("Example:")
    print(f"Part 1 expected {example_part1_expected} got {ex1}: {example_part1_expected == ex1}")
    print(f"Part 2 expected {example_part2_expected} got {ex2}: {example_part2_expected == ex2}")

    p1, p2 = process_file("input")
    print("Input:")
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")