def priority(r: str) -> int:
    if r > 'Z':
        return ord(r)-ord('a') + 1
    return ord(r)-ord('A') + 27

def part2(file: list[str]):
    sum = 0
    i = 0
    for i in range(0, len(file), 3):
        for c in file[i]:
            if c in file[i+1] and c in file[i+2]:
                sum += priority(c)
                break

    print(sum)