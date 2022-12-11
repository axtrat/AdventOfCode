def priority(r: str) -> int:
    if r > 'Z':
        return ord(r)-ord('a') + 1
    return ord(r)-ord('A') + 27

def part1(file: list[str]):
    sum = 0
    for line in file:
        first, second = set(line[:len(line)//2]), set(line[len(line)//2:])

        for c in first:
            if c in second:
                sum += priority(c)
    print(sum)