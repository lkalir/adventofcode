import adventofcode
import typing


class AOC2015Day2(adventofcode.AdventOfCodeSln):
    def __init__(self):
        super(AOC2015Day2, self).__init__(2, 2015,
                                          "I Was Told There Would Be No Math")

    class Rectangle(object):
        def __init__(self, length: int, width: int, height: int):
            self.length = length
            self.width = width
            self.height = height

        def smallest(self) -> typing.Tuple[int, int]:
            dims = [self.length, self.width, self.height]
            dims.remove(max(dims))
            return dims[0], dims[1]

        def surface_area(self) -> int:
            return 2 * (self.length * self.width + self.length * self.height +
                        self.width * self.height)

        def volume(self) -> int:
            return self.length * self.width * self.height

        def slack(self) -> int:
            dim1, dim2 = self.smallest()
            return dim1 * dim2

        def bow(self) -> int:
            dim1, dim2 = self.smallest()
            return 2 * (dim1 + dim2)

    def part1(self) -> int:
        tot = 0

        for line in self.input.splitlines():
            dims = map(lambda x: int(x), line.split("x"))
            rect = AOC2015Day2.Rectangle(*dims)
            tot += rect.surface_area() + rect.slack()

        return tot

    def part2(self) -> int:
        tot = 0

        for line in self.input.splitlines():
            dims = map(lambda x: int(x), line.split("x"))
            rect = AOC2015Day2.Rectangle(*dims)
            tot += rect.volume() + rect.bow()

        return tot
