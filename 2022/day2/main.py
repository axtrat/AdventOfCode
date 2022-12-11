import sys
import time

from part1 import part1
from part2 import part2


def main():
    filename = sys.argv[1]+".txt"

    with open(filename, "r") as file:
        lines = [line.rstrip() for line in file]
    
    start_time = time.time()
    part1(lines)
    print("--- {} ---".format(approx(time.time() - start_time)))

    start_time = time.time()
    part2(lines)
    print("--- {} ---".format(approx(time.time() - start_time)))
        


def approx(seconds: float):
    if seconds <= 0:
        return str("%.3f" % seconds)+"s"

    notation = ["s", "ms", "µs", "ns"]
    i = 0
    while seconds < 1:
        seconds *= 1000
        i += 1
    return str("%.3f" % seconds)+notation[i]

if __name__ == "__main__":
    main()