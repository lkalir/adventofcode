import adventofcode


class AOC2015Day1(adventofcode.AdventOfCodeSln):
    def __init__(self):
        super(AOC2015Day1, self).__init__(1, 2015, "Not Quite Lisp")

    def part1(self) -> int:
        up = self.input.count('(')
        return 2 * up - len(self.input)

    def part2(self) -> int:
        floor = 0

        for (idx, c) in enumerate(self.input, start=1):
            if c == '(':
                floor += 1
            else:
                floor -= 1

            if floor < 0:
                return idx

        return -1
