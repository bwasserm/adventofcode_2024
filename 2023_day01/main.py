# https://adventofcode.com/2023/day/1

import os

def part1(lines, expected=None):
    total = 0
    # Start of implementation #
    for line in lines:
        for c in line:
            if c.isdigit():
                first = int(c)
                break
        for c in reversed(line):
            if c.isdigit():
                last = int(c)
                break
        total += 10 * first + last
    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def get_digit(string):
    val = None
    if string.startswith("0"):
        val = 0
    elif string.startswith("1") or string.startswith("one"):
        val = 1
    elif string.startswith("2") or string.startswith("two"):
        val = 2
    elif string.startswith("3") or string.startswith("three"):
        val = 3
    elif string.startswith("4") or string.startswith("four"):
        val = 4
    elif string.startswith("5") or string.startswith("five"):
        val = 5
    elif string.startswith("6") or string.startswith("six"):
        val = 6
    elif string.startswith("7") or string.startswith("seven"):
        val = 7
    elif string.startswith("8") or string.startswith("eight"):
        val = 8
    elif string.startswith("9") or string.startswith("nine"):
        val = 9
    return val

def part2(lines, expected=None):
    total = 0
    # Start of implementation #
    for line in lines:
        first = 0
        for idx in range(len(line)):
            if first := get_digit(line[idx:]):
                break
        last = 0
        for idx in range(len(line) - 1, -1, -1):
            if last := get_digit(line[idx:]):
                break
        total += 10 * first + last
    # End of implementation #
    if expected is not None:
        assert total == expected
    return total


def process_file(filename, expected_part1=None, expected_part2=None):
    path = os.path.join(os.path.dirname(__file__), filename)
    with open(path, "r") as input_file:
        lines = input_file.readlines()

    p1 = part1(lines, expected_part1)
    p2 = part2(lines, expected_part2)
    return p1, p2


if __name__ == "__main__":
    example_part1_expected = 142
    example_part2_expected = 281
    ex1, _ = process_file("example1", example_part1_expected, None)
    _, ex2 = process_file("example2", None, example_part2_expected)
    print("Example:")
    print(f"Part 1 expected {example_part1_expected} got {ex1}: {example_part1_expected == ex1}")
    print(f"Part 2 expected {example_part2_expected} got {ex1}: {example_part2_expected == ex2}")

    p1, p2 = process_file("input")
    print("Input:")
    print(f"Part 1: {p1}")
    print(f"Part 2: {p2}")