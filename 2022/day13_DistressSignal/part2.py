from functools import cmp_to_key


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

def part2(file: list[str]):
    l = [[[2]],[[6]]]
    for i in range(0, len(file), 3):
        l.append(eval(file[i]))
        l.append(eval(file[i+1]))
    
    l.sort(key=cmp_to_key(inOrder), reverse=True)

    print((l.index([[2]])+1) * (l.index([[6]])+1))
    