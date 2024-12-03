import sys
import re

def part1(file: str):
    tuples = re.findall(r"mul\((\d+),(\d+)\)",file)

    res = sum(map(lambda tuple: int(tuple[0])*int(tuple[1]), tuples))
    
    print(res)

def part2(file: str):
    pattern = re.compile(r"(do\(\)|don't\(\))|mul\((\d+),(\d+)\)")

    enable = True
    res = 0
    for mt in re.finditer(pattern, file):
        match list(filter(lambda x: x != None, mt.groups())), enable:
            case ["do()"], _: enable = True
            case ["don't()"], _: enable = False
            case [a,b], True: res += int(a)*int(b)

    print(res)


with open(sys.argv[1]+".txt", "r") as file:
    lines = file.read()

part1(lines)
part2(lines)