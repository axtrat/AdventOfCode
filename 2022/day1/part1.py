def part1(file):
    elves = list[int]()
    summ = 0
    for line in file:
        if len(line) > 0:
            summ += int(line)
        else:
            elves.append(summ)
            summ = 0

    print(max(elves))
