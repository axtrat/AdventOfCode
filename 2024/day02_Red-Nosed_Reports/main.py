import sys


def isSafe(report: list[int]) -> bool:
    sign = -1 if report[0] < report[-1] else 1
    for i in range(len(report)-1):
        diff = (report[i] - report[i+1]) * sign
        if diff < 1 or diff > 3:
            return False
    return True


def part1(file: list[str]):
    #reports = [[int(num) for num in line.split()] for line in file]
    reports = list(map(lambda line: list(map(int, line.split())), file))

    res = len(list(filter(isSafe, reports)))
    print(res)

def part2(file: list[str]):
    pass
    #print(res)

def main():
    filename = sys.argv[1]+".txt"

    with open(filename, "r") as file:
        lines = [line.rstrip("\n") for line in file]
    
    
    part1(lines)
    part2(lines)


if __name__ == "__main__":
    main()