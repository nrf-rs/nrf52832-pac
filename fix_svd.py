#!/usr/bin/env python3.8
import re
from typing import List, Optional, Tuple
from dataclasses import dataclass


lsb_re = re.compile(r"^\s+<lsb>(\d)</lsb>\s+$")
msb_re = re.compile(r"^\s+<msb>(\d)</msb>\s+$")
name_re = re.compile(r"^\s+<name>(\w+)</name>\s+$")


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
    name: str
    reg_range: Tuple[int, int]
    field: Optional[TaskField]

    @staticmethod
    def from_lines(i: int, lines: List[str]):
        # extract register name
        name_match = name_re.match(lines[i])
        if name_match is None:
            raise ValueError(f"register at line {i} has no name")
        name = name_match.group(1)

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
        return Task(name, reg_range, field)


def extract_tasks(lines: List[str]) -> List[Task]:
    tasks = []

    for i, line in enumerate(lines):
        if "<name>TASKS_" not in line:
            continue
        if "<register>" not in lines[i-1]:
            continue

        task = Task.from_lines(i, lines)
        tasks.append(task)

    return tasks


def check_task_assumptions():
    """ check if every TASKS_ register has a field with the same
        name that has a single bit (lsb and msb, both zero)
    """
    file = open("nrf52840.svd", "r")
    lines = file.readlines()
    tasks = extract_tasks(lines)

    for task in tasks:
        if task.field is None:
            print("some tasks have no task field")
            continue

        if task.field.lsb != 0:
            print("not all tasks have lsb = 0")
        if task.field.msb != 0:
            print("not all tasks have msb = 0")


def to_insert(taskname: str, num_spaces: int) -> List[str]:
    indent = "".ljust(num_spaces)
    return ([
        indent + "  <fields>\n",
        indent + "    <field>\n",
        indent + f"     <name>{taskname}</name>\n",
        indent + "      <lsb>0</lsb>\n",
        indent + "      <msb>0</msb>\n",
        indent + "    </field>\n",
        indent + "  </fields>\n"])


if __name__ == "__main__":
    check_task_assumptions()

    file = open("nrf52832.svd", "r")
    lines = file.readlines()
    tasks = extract_tasks(lines)

    # this assumes the fields key is not present
    for task in reversed(tasks):
        pos = task.reg_range[1] - 1
        reg_line = lines[task.reg_range[1]]
        indent = len(reg_line) - len(reg_line.lstrip(" "))
        ins = to_insert(task.name, indent)
        # print(ins)
        lines[pos:pos] = ins

    file = open("nrf52832_fixed.svd", "w")
    file.writelines(lines)
