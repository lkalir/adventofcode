import pytest
from adventofcode.aoc_2015 import *


def test_aoc_2015_day_1_part_1():
    dut = AOC2015Day1()
    assert 138 == dut.part1()


def test_aoc_2015_day_1_part_2():
    dut = AOC2015Day1()
    assert 1771 == dut.part2()


def test_aoc_2015_day_1_part_1_bench(benchmark):
    dut = AOC2015Day1()
    benchmark(dut.part1)


def test_aoc_2015_day_1_part_2_bench(benchmark):
    dut = AOC2015Day1()
    benchmark(dut.part2)
