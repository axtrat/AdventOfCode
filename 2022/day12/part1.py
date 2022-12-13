import math


class Graph:
    __adjacent = dict()

    def addEdge(self, a, b):
        if a not in self.__adjacent:
            self.__adjacent[a] = [a]
        self.__adjacent[a].append(b)
        if a != b:
            if b not in self.__adjacent:
                self.__adjacent[b] = []

    def bfs(self, start, goal):
        visited = set()
        tovisit = [start]
        came_from = {start: 0}

        while len(tovisit) > 0:
            x = tovisit.pop(0)

            if x == goal:
                return came_from[x]

            visited.add(x)
            for y in self.__adjacent[x]:
                if y in visited:
                    continue

                if y not in tovisit:
                    tovisit.append(y)

                came_from[y] = came_from[x]+1

        return 0


def reacble(mapp: list[list[str]], i: int, j: int) -> list[tuple[int, int]]:
    v = mapp[i][j]
    res = list[tuple[int, int]]()
    if i+1 < len(mapp) and mapp[i+1][j] - v <= 1:
        res.append((i+1, j))
    if i-1 >= 0 and mapp[i-1][j] - v <= 1:
        res.append((i-1, j))
    if j+1 < len(mapp[i]) and mapp[i][j+1] - v <= 1:
        res.append((i, j+1))
    if j-1 >= 0 and mapp[i][j-1] - v <= 1:
        res.append((i, j-1))

    return res


def part1(file: list[str]):
    mapp = []
    for i in range(len(file)):
        mapp.append([])
        for j in range(len(file[i])):
            mapp[i].append(ord(file[i][j])-ord("a"))

            if file[i][j] == "S":
                start = (i, j)
                mapp[i][j] = 0

            elif file[i][j] == "E":
                goal = (i, j)
                mapp[i][j] = ord("z")-ord("a")

    g = Graph()

    for i in range(len(mapp)):
        for j in range(len(mapp[i])):
            for p in reacble(mapp, i, j):
                g.addEdge((i, j), p)

    print(g.bfs(start, goal))
