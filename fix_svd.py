#!/usr/bin/env python3.8
import re
from typing import List, Optional, Tuple
from dataclasses import dataclass


lsb_re = re.compile(r"^\s+<lsb>(\d)</lsb>\s+$")
msb_re = re.compile(r"^\s+<msb>(\d)</msb>\s+$")
name_re = re.compile(r"^\s+<name>([\w|\[%\]]+)</name>\s+$")


def indent(line: str) -> int:
    return len(line) - len(line.lstrip(" "))


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
    field_name: str
    reg_range: Tuple[int, int]
    field: Optional[TaskField]

    @staticmethod
    def from_lines(reg_pos: int, name_pos: int, lines: List[str]):
        # extract register name
        name_match = name_re.match(lines[name_pos])
        if name_match is None:
            raise ValueError(f"register at line {reg_pos} has no name")
        reg_name = name_match.group(1)
        if reg_name is None:
            raise ValueError(f"register at line {reg_pos} has no name")

        # find end of register
        reg_range = None
        for j, line in enumerate(lines[reg_pos:]):
            if "</register>" in line:
                reg_range = (reg_pos, reg_pos+j+1)
                break

        if reg_range is None:
            raise ValueError(f"register at line {reg_pos} has no end")

        # has field?
        field_name = reg_name
        if reg_name.endswith("[%s]"):  # field name is then without "[%s]"
            field_name = reg_name[:-len("[%s]")]

        field = None
        lines = lines[reg_range[0]:reg_range[1]]
        for j, line in enumerate(lines):
            if "<field>" not in line:
                continue
            if "<name>"+field_name not in lines[j+1]:
                continue
            field = TaskField.from_lines(lines[j:])
            break

        return Register(reg_name, field_name, reg_range, field)


def register_start(lines: List[str], name_pos: int) -> Optional[int]:
    name_indent = indent(lines[name_pos])
    lines = lines[max(name_pos-10, 0):name_pos]
    for j, line in enumerate(reversed(lines)):
        if indent(line) == name_indent:
            continue
        if "<register>" in line:
            return name_pos - j - 1
        break

    return None


def extract_registers(name_prompt: str, lines: List[str]) -> List[Register]:
    tasks = []

    for i, line in enumerate(lines):
        if name_prompt not in line:
            continue

        name_pos = i
        reg_pos = register_start(lines, name_pos)
        if reg_pos is None:
            continue

        task = Register.from_lines(reg_pos, name_pos, lines)
        tasks.append(task)

    return tasks


def check_assumptions(name_prompt: str):
    """ check if every TASKS_ register has a field with the same
        name that has a single bit (lsb and msb, both zero)
    """
    file = open("nrf52840.svd", "r")
    lines = file.readlines()
    # lines = lines[4873:4888]
    # lines = lines[40832:40846]

    tasks = extract_registers(name_prompt, lines)

    for task in tasks:
        if task.field is None:
            print(f"register {task.name} at {task.reg_range[0]} have no field")
            continue

        if task.field.lsb != 0:
            print(f"register {task.name} at {task.reg_range[0]} has lsb != 0")
        if task.field.msb != 0:
            print(f"register {task.name} at {task.reg_range[0]} has msb != 0")


def to_insert(field_name: str, num_spaces: int) -> List[str]:
    indent = "".ljust(num_spaces)
    return ([
        indent + "  <fields>\n",
        indent + "    <field>\n",
        indent + f"     <name>{field_name}</name>\n",
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
    for reg in reversed(registers):
        pos = reg.reg_range[1] - 1
        reg_line = lines[reg.reg_range[1]]
        lines[pos:pos] = to_insert(reg.field_name, indent(reg_line))

    file = open("nrf52832_fixed.svd", "w")
    file.writelines(lines)
