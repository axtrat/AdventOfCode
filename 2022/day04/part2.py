def overlap(a, b, c, d):
    if a and b in range(c, d+1): 
        return True
    return False

def part2(file: list[str]):
    count = 0

    for line in file:
        tmp = [[int(x) for x in segment.split("-")] for segment in line.split(",")]
        a, b = tmp[0]
        c, d = tmp[1]
        if overlap(a, b, c, d) or overlap(c, d, a, b):
            count+=1

    print(count)