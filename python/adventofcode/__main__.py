import argparse

from adventofcode.aoc_2015 import AOC2015Day1, AOC2015Day2

SLNS = {2015: {1: AOC2015Day1, 2: AOC2015Day2}}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("day", type=int, help="Day")
    parser.add_argument("year", type=int, help="Year")

    args = parser.parse_args()

    try:
        d = SLNS[args.year][args.day]()
        print(f"{args.year} day {args.day} {d.name} part 1: {d.part1()}")
        print(f"{args.year} day {args.day} {d.name} part 2: {d.part2()}")
    except Exception:
        print("Unsupported day")


if __name__ == "__main__":
    main()
