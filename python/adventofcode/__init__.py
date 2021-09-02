from ctypes import CDLL, c_char_p
from abc import ABC, abstractmethod

_LIBAOC = CDLL("libaocdata.so")
_GET_INPUTS = _LIBAOC.get_inputs
_GET_INPUTS.restype = c_char_p


def get_inputs(day: int, year: int) -> str:
    """Gets the Advent of Code input for the given day and year

    Raises
    ------
    AttributeError
        If the day and year combination is currently unsupported
    """
    return _GET_INPUTS(day, year).decode("UTF-8")


class AdventOfCodeSln(ABC):
    def __init__(self, day: int, year: int, name: str):
        self.day = day
        self.year = year
        self.name = name
        self.input = get_inputs(day, year)

    @abstractmethod
    def part1(self) -> int:
        pass

    @abstractmethod
    def part2(self) -> int:
        pass
