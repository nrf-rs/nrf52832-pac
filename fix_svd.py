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
class Register:
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
            if "<name>"+name not in lines[j+1]:
                continue
            field = TaskField.from_lines(lines[j:])
            break
        return Register(name, reg_range, field)


def extract_registers(name_prompt: str, lines: List[str]) -> List[Register]:
    tasks = []

    for i, line in enumerate(lines):
        # if "<name>TASKS_" not in line:
        if name_prompt not in line:
            continue
        if "<register>" not in lines[i-1]:
            continue

        task = Register.from_lines(i, lines)
        tasks.append(task)

    return tasks


def check_assumptions(name_prompt: str):
    """ check if every TASKS_ register has a field with the same
        name that has a single bit (lsb and msb, both zero)
    """
    file = open("nrf52840.svd", "r")
    lines = file.readlines()
    tasks = extract_registers(name_prompt, lines)

    for task in tasks:
        if task.field is None:
            print("some registers have no field")
            continue

        if task.field.lsb != 0:
            print("not all registers have lsb = 0")
        if task.field.msb != 0:
            print("not all registers have msb = 0")


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


TASK_PROMT = "<name>TASKS_"
EVENT_PROMT = "<name>EVENTS_"
if __name__ == "__main__":
    check_assumptions(TASK_PROMT)
    check_assumptions(EVENT_PROMT)

    file = open("nrf52832.svd", "r")
    lines = file.readlines()
    registers = extract_registers(TASK_PROMT, lines)
    registers += extract_registers(EVENT_PROMT, lines)
    registers = sorted(registers, key=lambda r: r.reg_range[0])

    # this assumes the fields key is not present
    for task in reversed(registers):
        pos = task.reg_range[1] - 1
        reg_line = lines[task.reg_range[1]]
        indent = len(reg_line) - len(reg_line.lstrip(" "))
        lines[pos:pos] = to_insert(task.name, indent)

    file = open("nrf52832_fixed.svd", "w")
    file.writelines(lines)
