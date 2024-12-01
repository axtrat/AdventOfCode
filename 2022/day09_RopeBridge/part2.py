import math


DIRECTIONS = {
    "U": (0, 1),
    "D": (0, -1),
    "R": (1, 0),
    "L": (-1, 0)
}


def distance(p1: tuple[int, int], p2: tuple[int, int]):
    x, y = p1[0] - p2[0], p1[1] - p2[1]
    return math.sqrt(x*x+y*y)


def sign(x) -> int:
    if x > 0:
        return 1
    elif x < 0:
        return -1
    return 0


def catch(rope):
    for i in range(len(rope)-1):
        if distance(rope[i], rope[i+1]) >= 2:
            rope[i+1][0] += sign((rope[i][0] - rope[i+1][0]) / 2)
            rope[i+1][1] += sign((rope[i][1] - rope[i+1][1]) / 2)
        else:
            break


def move(rope, visited, m):
    step = DIRECTIONS[m[0]]
    for i in range(int(m[1])):

        rope[0][0] += step[0]
        rope[0][1] += step[1]
        catch(rope)
        visited.add(tuple(rope[-1]))


def part2(file: list[str]):
    rope = [[0, 0] for i in range(10)]
    visited = {(0, 0)}

    for line in file:
        m = line.split()
        move(rope, visited, m)

    print(len(visited))
