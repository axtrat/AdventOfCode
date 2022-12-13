def move(stacks: list[list[str]], f, t):
    stacks[t].append(stacks[f].pop())


def part1(file: list[str]):
    stacks = [[] for x in range((len(file[0]) + 1) // 4)]

    for i, line in enumerate(file):
        if "[" not in line:
            break
        for j, stack in enumerate(stacks):
            if line[j*4+1] != " ":
                stack.insert(0, line[j*4+1])

    for line in file[i+2:]:
        _, n, _, f, _, t = line.split()
        for i in range(int(n)):
            move(stacks, int(f)-1, int(t)-1)

    for v in stacks:
        print(v[-1], end="")
    print()
