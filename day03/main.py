# https://adventofcode.com/2024/day/3
# https://xkcd.com/208/

import os
import re

def part1(lines: str, expected: int | None = None) -> int:
    # Start of implementation #
    pattern = re.compile(r"mul\((\d+),(\d+)\)")
    total = sum([int(m.group(1)) * int(m.group(2)) for m in pattern.finditer(lines)])
    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def part2(lines: str, expected: int | None = None) -> int:
    # Start of implementation #
    domul = re.compile(r"(do\(\)|don't\(\)|mul\(\d+,\d+\))")
    mul = re.compile(r"mul\((\d+),(\d+)\)")
    enabled = True
    total = 0
    for action in domul.finditer(lines):
        if action.group(0) == "do()":
            enabled = True
            continue
        elif action.group(0) == "don't()":
            enabled = False
            continue
        elif enabled:
            for m in mul.finditer(action.group(0)):
                total += int(m.group(1)) * int(m.group(2))

    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def process_file(filename: str, expected_part1: int | None = None, expected_part2: int | None = None):
    path = os.path.join(os.path.dirname(__file__), filename)
    with open(path, "r") as input_file:
        lines = input_file.read()

    p1 = part1(lines, expected_part1)
    p2 = part2(lines, expected_part2)
    return p1, p2


if __name__ == "__main__":
    example_part1_expected = 161
    example_part2_expected = 48
    ex1, ex2 = process_file("example", example_part1_expected, example_part2_expected)
    print("Example:")
    print(f"Part 1 expected {example_part1_expected} got {ex1}: {example_part1_expected == ex1}")
    print(f"Part 2 expected {example_part2_expected} got {ex2}: {example_part2_expected == ex2}")

    p1, p2 = process_file("input")
    print("Input:")
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")