def move(stacks: list[list[str]], f, t, n=1):
    stacks[t].extend(stacks[f][-n:])
    stacks[f] = stacks[f][:-n]

def readStack(file, stacks):
    for line in file:
        if "[" not in line:
            break
        for i in range(len(stacks)):
            if line[i*4+1] != " ":
                stacks[i].insert(0, line[i*4+1])
            i += 1
    return i+1

def part2(file: list[str]):
    stacks = [[] for x in range((len(file[0]) + 1) // 4)]

    i = readStack(file, stacks)

    for line in file[i:]:
        _, n, _, f, _, t = line.split()
        move(stacks, int(f)-1, int(t)-1, int(n))

    for v in stacks:
        print(v[-1], end="")
    print()
	