import sys

sys.path.append('../')
from utilities import Timer

from part1 import part1
from part2 import part2

def main():
    filename = sys.argv[1]+".txt"
    timer = Timer()

    with open(filename, "r") as file:
        lines = [line.rstrip("\n") for line in file]
    
    timer.start()
    part1(lines)
    print(timer)

    timer.start()
    part2(lines)
    print(timer)


if __name__ == "__main__":
    main()