# https://adventofcode.com/2023/day/4

import os
import itertools
from typing import List

def part1(lines: List[str], expected: int | None = None) -> int:
    total = 0
    # Start of implementation #
    num_rows = len(lines)
    num_cols = len(lines[0].strip())
    mas = "MAS"
    for row_dir, col_dir in itertools.product((-1, 0, 1), repeat=2):
        for row_start, col_start in itertools.product(range(num_rows), range(num_cols)):
            if lines[row_start][col_start] == "X":
                for step in range(3):
                    row = row_start + row_dir * (step + 1)
                    col = col_start + col_dir * (step + 1)
                    if 0 <= row < num_rows and 0 <= col < num_cols:
                        if lines[row][col] == mas[step]:
                            if step == 2:
                                total += 1
                        else:
                            break
                    else:
                        break
    # End of implementation #
    if expected is not None:
        print(f"Part 1: Expected: {expected} Calculated: {total} Equal: {expected == total}")
    return total


def part2(lines: List[str], expected: int | None = None) -> int:
    total = 0
    # Start of implementation #
    num_rows = len(lines)
    num_cols = len(lines[0].strip())
    for row_start, col_start in itertools.product(range(1, num_rows - 1), range(1, num_cols - 1)):
        if lines[row_start][col_start] == "A":
            if (((lines[row_start -  1][col_start - 1] == 'M' and lines[row_start +  1][col_start + 1] == 'S') or
                 (lines[row_start -  1][col_start - 1] == 'S' and lines[row_start +  1][col_start + 1] == 'M')) and
                ((lines[row_start -  1][col_start + 1] == 'M' and lines[row_start +  1][col_start - 1] == 'S') or
                 (lines[row_start -  1][col_start + 1] == 'S' and lines[row_start +  1][col_start - 1] == 'M'))):
                total += 1
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
    example_part1_expected = 18
    example_part2_expected = 9
    ex1, ex2 = process_file("example", example_part1_expected, example_part2_expected)
    print("Example:")
    print(f"Part 1 expected {example_part1_expected} got {ex1}: {example_part1_expected == ex1}")
    print(f"Part 2 expected {example_part2_expected} got {ex2}: {example_part2_expected == ex2}")

    p1, p2 = process_file("input")
    print("Input:")
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")