import re
import sys
from collections import Counter


def part1(file: list[str]):
    l1, l2 = zip(*(re.split("\s+", line) for line in file))
    l1, l2 = sorted(l1), sorted(l2)

    res = sum([abs(int(l1[i]) - int(l2[i])) for i in range(len(l1))])
    
    print(res)

def part2(file: list[str]):
    l1, l2 = zip(*(re.split("\s+", line) for line in file))
    dict2 = Counter(l2)

    res = sum(int(num) * dict2[num] for num in l1 if num in dict2)

    print(res)
    

def main():
    filename = sys.argv[1]+".txt"

    with open(filename, "r") as file:
        lines = [line.rstrip("\n") for line in file]
    
    
    part1(lines)
    part2(lines)


if __name__ == "__main__":
    main()