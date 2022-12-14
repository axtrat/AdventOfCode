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
        came_from = dict()

        while len(tovisit) > 0:
            x = tovisit.pop(0)

            if x == goal:
                return self.reconstruct_path(came_from,goal)

            visited.add(x)
            for y in self.__adjacent[x]:
                if y in visited:
                    continue

                if y not in tovisit:
                    tovisit.append(y)

                came_from[y] = x

        return []
    
    def reconstruct_path(self,came_from,current_node):
        res = []
        while current_node in came_from:
            res.insert(0, current_node)
            current_node = came_from[current_node]
        return res


def reacble(mapp: list[list[int]], i: int, j: int) -> list[tuple[int, int]]:
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

def smart(mapp, path, starts) -> int:
    pathlen = len(path)
    min = pathlen
    for i, node in enumerate(path):
        if mapp[node[0]][node[1]] == 0:
            min = pathlen-i
            if node in starts:
                starts.remove(node)
        else:
            break
    return min


def part2(file: list[str]):
    mapp = []
    starts = []
    for i in range(len(file)):
        mapp.append([])
        for j, c in enumerate(file[i]):
            mapp[i].append(ord(c)-ord("a"))

            if file[i][j] == "a":
                starts.append((i, j))

            if c == "S":
                start = (i, j)
                mapp[i][j] = 0

            elif c == "E":
                goal = (i, j)
                mapp[i][j] = ord("z")-ord("a")


    g = Graph()

    for i in range(len(mapp)):
        for j in range(len(mapp[i])):
            for p in reacble(mapp, i, j):
                g.addEdge((i, j), p)

    
    min = smart(mapp, g.bfs(start, goal), starts)
    while len(starts) > 0:
        start = starts.pop()
        pathlen = smart(mapp, g.bfs(start, goal), starts)
        if pathlen != 0 and pathlen < min:
            min = pathlen

    print(min)
