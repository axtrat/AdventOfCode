import re
import sys
from collections import Counter


def part1(file: list[str]):
    l1, l2 = zip(*(re.split("\s+", line) for line in file))
    l1, l2 = list(l1), list(l2)

    l1.sort()
    l2.sort()

    sum = 0
    for i in range(len(l1)):
        sum += abs(int(l1[i]) - int(l2[i]))
    
    print(sum)

def part1(file: list[str]):
    l1, l2 = zip(*(re.split("\s+", line) for line in file))
    dict1, dict2 = Counter(l1), Counter(l2)

    sum = 0
    for key in dict1:
        if key in dict2:
            sum += abs(int(key) * dict1[key] * dict2[key])

    print(sum)
    

def main():
    filename = sys.argv[1]+".txt"

    with open(filename, "r") as file:
        lines = [line.rstrip("\n") for line in file]
    
    
    part1(lines)


if __name__ == "__main__":
    main()