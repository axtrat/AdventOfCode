def move(stacks: list[list[str]], f, t, n=1):
    stacks[t].extend(stacks[f][-n:])
    stacks[f] = stacks[f][:-n]


def part2(file: list[str]):
    stacks = [[] for x in range((len(file[0]) + 1) // 4)]

    for i, line in enumerate(file):
        if "[" not in line:
            break
        for j, stack in enumerate(stacks):
            if line[j*4+1] != " ":
                stack.insert(0, line[j*4+1])

    for line in file[i+2:]:
        _, n, _, f, _, t = line.split()
        move(stacks, int(f)-1, int(t)-1, int(n))

    for v in stacks:
        print(v[-1], end="")
    print()
