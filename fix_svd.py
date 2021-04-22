#!/usr/bin/env python3.8
import re
from typing import List, Optional, Tuple
from dataclasses import dataclass
from collections import namedtuple


lsb_re = re.compile(r"^\s+<lsb>(\d)</lsb>\s+$")
msb_re = re.compile(r"^\s+<msb>(\d)</msb>\s+$")


@dataclass
class TaskField:
    lsb: int
    msb: int

    @staticmethod
    def from_lines(lines: List[str]):
        lsb = lsb_re.match(lines[2])
        msb = msb_re.match(lines[3])
        if lsb is None or msb is None:
            return None

        field = TaskField(int(lsb.group(1)), int(msb.group(1)))
        return field


@dataclass
class Task:
    reg_range: Tuple[int, int]
    field: Optional[TaskField]

    @staticmethod
    def from_lines(i: int, lines: List[str]):
        # find end of register
        reg_range = None
        for j, line in enumerate(lines[i:]):
            if "</register>" in line:
                reg_range = (i, i+j+1)
                break

        if reg_range is None:
            raise ValueError(f"register at line {i} has no end")

        # has field?
        field = None
        lines = lines[reg_range[0]:reg_range[1]]
        for j, line in enumerate(lines):
            if "<field>" not in line:
                continue
            if "<name>TASKS_" not in lines[j+1]:
                continue
            field = TaskField.from_lines(lines[j:])
            break
        return Task(reg_range, field)


if __name__ == "__main__":

    file = open("nrf52840.svd", "r")
    lines = file.readlines()
    tasks = []

    for i, line in enumerate(lines):
        if "<name>TASKS_" not in line:
            continue
        if "<register>" not in lines[i-1]:
            continue

        task = Task.from_lines(i, lines)
        tasks.append(task)

    for task in tasks:
        if task.field.lsb != 0:
            print("not all tasks have lsb = 0")
        if task.field.msb != 0:
            print("not all tasks have msb = 0")
