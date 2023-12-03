def printMap(mapp, depth):
    _, x = zip(*mapp)
    minx, maxx = min(x), max(x)


    for i in range(depth+1):
        print("%3d"%i, end=" ")
        for j in range(minx, maxx+1):
            if (i, j) in mapp:
                print(mapp[(i, j)], end="")
            else:
                print(end=".")
        print()
def sandFall(rocks, path, sand, maxRow):
    while True:
        prev = tuple(sand)
        
        sand[0] += 1
        if sand[0] > maxRow:
            return True
        if tuple(sand) in rocks:
            if (sand[0], sand[1]-1) not in rocks:
                sand[1] -= 1
            elif (sand[0], sand[1]+1) not in rocks:
                sand[1] += 1
            else:
                sand[0] -= 1
                rocks[prev] = "o"
                return False
            
        path[tuple(sand)] = prev
        

def sandFalls(rocks, maxRow):
    path = dict()
    i, sand = 0, [0, 500]
    while True:
        if sandFall(rocks, path, sand, maxRow):
            break
        prev = path[tuple(sand)]
        sand[0], sand[1] = prev[0], prev[1]
        i+=1
    return i

def draw(mapp, p1, p2):
    for i in range(p1[1], p2[1]+1):
        for j in range(p1[0], p2[0]+1):
            mapp[(i,j)] = "#"

def part1(file: list[str]):
    mapp = dict()
    depth = 0
    for line in file:
        points = [[int(x) for x in ps.split(",")] for ps in line.split(" -> ")]

        for i in range(1, len(points)):
            p1, p2 = sorted(points[i-1:i+1])
            depth = max(p2[1], depth)
            draw(mapp, p1, p2)

    print(sandFalls(mapp, depth))