import argparse
import os
import re

def part1(lines):
    total = 0
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
    print(f"Part 1: {total}")


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

def part2(lines):
    total = 0
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
    print(f"Part 2: {total}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("file", type=str)
    args = parser.parse_args()

    filename = os.path.join(os.path.dirname(__file__), args.file)
    with open(filename, "r") as input_file:
        lines = input_file.readlines()

    print()
    part1(lines)
    part2(lines)