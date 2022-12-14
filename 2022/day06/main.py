import sys

sys.path.append('../')
from utilities import Timer

from part1 import part1
from part2 import part2

def main():
    filename = sys.argv[1]+".txt"

    with open(filename, "r") as file:
        lines = [line.rstrip("\n") for line in file]
    
    timer = Timer()
    part1(lines)
    print(timer)

    timer.restart()
    part2(lines)
    print(timer)


if __name__ == "__main__":
    main()