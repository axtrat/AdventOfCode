def isFullyIn(a, b, c, d):
    if c >= a and d <= b:
        return True
    return False

def part1(file: list[str]):
    count = 0

    for line in file:
        segments = [[int(x) for x in segment.split("-")] for segment in line.split(",")]
        a, b = segments[0]
        c, d = segments[1]
        if isFullyIn(a, b, c, d) or isFullyIn(c, d, a, b):
            count+=1

    print(count)