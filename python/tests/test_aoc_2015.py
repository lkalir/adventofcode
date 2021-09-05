import pytest
from adventofcode.aoc_2015 import *


def test_2015_1_1():
    dut = AOC2015Day1()
    assert 138 == dut.part1()


def test_2015_1_2():
    dut = AOC2015Day1()
    assert 1771 == dut.part2()


def test_2015_1_1_bench(benchmark):
    dut = AOC2015Day1()
    benchmark(dut.part1)


def test_2015_1_2_bench(benchmark):
    dut = AOC2015Day1()
    benchmark(dut.part2)


def test_2015_2_1():
    dut = AOC2015Day2()
    assert 1588178 == dut.part1()


def test_2015_2_2():
    dut = AOC2015Day2()
    assert 3783758 == dut.part2()


def test_2015_2_1_bench(benchmark):
    dut = AOC2015Day2()
    benchmark(dut.part1)


def test_2015_2_2_bench(benchmark):
    dut = AOC2015Day2()
    benchmark(dut.part2)
