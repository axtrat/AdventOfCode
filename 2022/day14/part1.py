def printMap(mapp):
    for i, row in enumerate(mapp):
        print("%3d"%i, end=" ")
        for c in row:
            print(c, end="")
        print()

def dropp(mapp, s) -> bool:
    while mapp[s[1]][s[0]] == ".":
        s[1]+=1
        if s[1] >= len(mapp):
            return True

    if s[0]-1 < 0 or s[0]+1 >= len(mapp[s[1]]):
        return True

    if mapp[s[1]][s[0]-1] == ".":
        s[0] = s[0]-1
        return dropp(mapp, s)
    elif mapp[s[1]][s[0]+1] == ".":
        s[0] = s[0]+1
        return dropp(mapp, s)
    s[1]-=1


def expandMap(mapp, t, l, r, minx, maxx):
    if t >= len(mapp):
        for _ in range(len(mapp), t+1):
            mapp.append([])

    if r >= maxx:
        maxx = r
    
    for i in range(len(mapp)):
        if maxx-minx >= len(mapp[i]) :
            for _ in range(len(mapp[i]), maxx+1-minx):
                mapp[i].append(".")
        if l < minx:
            for _ in range(l, minx):
                mapp[i].insert(0, ".")
    
    if l < minx:
        minx = l
    
    return minx, maxx

def draw(mapp, p1, p2, minx):
    for i in range(p1[1], p2[1]+1):
        for j in range(p1[0]-minx, p2[0]+1-minx):
            mapp[i][j] = "#"

def part1(file: list[str]):
    mapp = list()
    minx, maxx = 500, 0
    for line in file:
        points = [[int(x) for x in ps.split(",")] for ps in line.split(" -> ")]

        for i in range(1, len(points)):
            p1, p2 = sorted(points[i-1:i+1])
            minx, maxx = expandMap(mapp, p2[1], p1[0], p2[0], minx, maxx)
            draw(mapp, p1, p2, minx)
    
    i, s = 0, [500-minx, 0]
    while not dropp(mapp, s):
        mapp[s[1]][s[0]] = "o"
        i, s = i+1, [500-minx, 0]
    
    print(i)