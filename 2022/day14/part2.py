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

def part2(file: list[str]):
    mapp = list()
    segments = list()
    minx, maxx = 500, 0
    for line in file:
        points = [[int(x) for x in ps.split(",")] for ps in line.split(" -> ")]

        for i in range(1, len(points)):
            if points[i-1][1] < points[i][1]:
                d, t = points[i-1][1], points[i][1]
            else:
                d, t = points[i][1], points[i-1][1]

            if t >= len(mapp):
                for _ in range(len(mapp), t+1):
                    mapp.append([])

            if points[i-1][0] < points[i][0]:
                l, r = points[i-1][0], points[i][0] 
            else:
                l, r = points[i][0], points[i-1][0]
            

            if r >= maxx:
                maxx = r
            
            for i in range(len(mapp)):
                if l < minx:
                    for _ in range(l, minx):
                        mapp[i].insert(0, ".")
                if maxx-minx >= len(mapp[i]) :
                    for _ in range(len(mapp[i]), maxx+1-minx):
                        mapp[i].append(".")
            if l < minx:
                minx = l
            
            for i in range(d, t+1):
                for j in range(l-minx, r+1-minx):
                        mapp[i][j] = "#"

    mapp.append(["." for i in range(maxx+1-minx)])
    maxx = 500 + (len(mapp)+1 // 2)
    l = 500 - (len(mapp)+1 // 2)
    for i in range(len(mapp)):
        if maxx-minx >= len(mapp[i]) :
            for _ in range(len(mapp[i]), maxx+1-minx):
                mapp[i].append(".")
        if l < minx:
            for _ in range(l, minx):
                mapp[i].insert(0, ".")
    minx = l
    mapp.append(["#" for i in range(maxx+1-minx)])
    
    i, s = 0, [500-minx, 0]
    while not dropp(mapp, s):
        mapp[s[1]][s[0]] = "o"
        i, s = i+1, [500-minx, 0]

    print(i)