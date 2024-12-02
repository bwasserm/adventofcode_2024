# https://adventofcode.com/2024/day/3

import os
from typing import List


def parse_lines(lines: List[str]) -> List[List[int]]:
    reports = []
    for line in lines:
        levels = [int(l) for l in line.split()]
        reports.append(levels)
    return reports


def test_report(report: List[int]) -> bool:
    up = True
    down = True
    small = True
    for i, l in enumerate(report[1:]):
        up &= report[i] < l
        down &= report[i] > l
        small &= abs(report[i] - l) <= 3
    return small and (up ^ down)


def part1(lines: List[str], expected: int | None = None) -> int:
    # Start of implementation #
    reports = parse_lines(lines)
    total = sum([test_report(r) for r in reports])
    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def part2(lines: List[str], expected: int | None = None) -> int:
    # Start of implementation #
    reports = parse_lines(lines)
    total = 0
    for report in reports:
        if test_report(report):
            total += 1
        else:
            for i in range(len(report)):
                r2 = report[:i] + report[i+1:]
                if test_report(r2):
                    total += 1
                    break
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
    example_part1_expected = 2
    example_part2_expected = 4
    ex1, _ = process_file("example1", example_part1_expected, None)
    _, ex2 = process_file("example1", None, example_part2_expected)
    print("Example:")
    print(f"Part 1 expected {example_part1_expected} got {ex1}: {example_part1_expected == ex1}")
    print(f"Part 2 expected {example_part2_expected} got {ex2}: {example_part2_expected == ex2}")

    p1, p2 = process_file("input")
    print("Input:")
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")