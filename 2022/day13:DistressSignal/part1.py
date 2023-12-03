def inOrder(left: list|int, right: list|int) -> int:
    match left, right:
        case int(x), int(y): return y-x
        case int(x), _: return inOrder([x], right)
        case _, int(y): return inOrder(left, [y])
        case _:
            for i, e in enumerate(left):
                if i >= len(right):
                    return -1
                res = inOrder(e, right[i])
                if res != 0:
                    return res
            return len(right)-len(left)


def part1(file: list[str]):
    sum = 0
    for i in range(0, len(file), 3):
        left, right = eval(file[i]), eval(file[i+1])
        if inOrder(left, right) >= 0:
            sum += i//3+1
    print(sum)
