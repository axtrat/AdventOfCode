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


def move(head, tail, visited, m):
    step = DIRECTIONS[m[0]]
    for i in range(int(m[1])):
        prec = tuple(head)
        head[0] += step[0]
        head[1] += step[1]
        
        if distance(head, tail) >= 2:
            tail[0] = prec[0]
            tail[1] = prec[1]
        visited.add(tuple(tail))


def part1(file: list[str]):
    head, tail = [[0, 0] for i in range(2)]
    visited = {(0, 0)}

    for line in file:
        m = line.split()
        move(head, tail, visited, m)

    print(len(visited))