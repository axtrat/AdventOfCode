def max3(l:list[int]) -> list[int]:
    max = [0, 0, 0]
    for v in l:
        for i in range(3):
            if v > max[i]:
                max.insert(i, v)
                max = max[:-1]
                break
    return max

def part2(file):
    elves = list[int]()
    summ = 0
    for line in file:
        if len(line) > 0:
            summ += int(line)
        else:
            elves.append(summ)
            summ = 0

    print(sum(max3(elves)))