import sys
import re

def part1(file: list[str]):
    tuples = re.findall("mul\((\d+),(\d+)\)","".join(file))

    res = sum(map(lambda tuple: int(tuple[0])*int(tuple[1]), tuples))
    
    print(res)

def part2(file: list[str]):
    pattern = re.compile("do\(\)|don't\(\)|mul\((\d+),(\d+)\)")

    enable = True
    res = 0
    for mt in re.finditer(pattern, "".join(file)):
        match mt.group(), mt.groups():
            case ("do()", _):
                enable = True
            case ("don't()", _):
                enable = False
            case (_, (x, y)):
                if enable:
                    res += int(x) * int(y)
    
    print(res)


with open(sys.argv[1]+".txt", "r") as file:
    lines = [line.rstrip("\n") for line in file]

part1(lines)
part2(lines)