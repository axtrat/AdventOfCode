def move(stacks: list[list[str]], f, t):
    stacks[t].append(stacks[f].pop())


def readStack(file, stacks):
    for line in file:
        if "[" not in line:
            break
        for i in range(len(stacks)):
            if line[i*4+1] != " ":
                stacks[i].insert(0, line[i*4+1])
            i += 1
    return i+1


def part1(file: list[str]):
    stacks = [[] for x in range((len(file[0]) + 1) // 4)]

    i = readStack(file, stacks)

    for line in file[i:]:
        _, n, _, f, _, t = line.split()
        for i in range(int(n)):
            move(stacks, int(f)-1, int(t)-1)

    for v in stacks:
        print(v[-1], end="")
    print()
