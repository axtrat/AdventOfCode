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

def sandFall(mapp, s, depth) -> bool:
    while (s[0], s[1]) not  in mapp:
        s[0]+=1
        if s[0] > depth:
            return True
            
    if (s[0], s[1]-1) not in mapp:
        s[1] -=1
        return sandFall(mapp, s, depth)
    elif(s[0], s[1]+1) not in mapp:
        s[1] += 1
        return sandFall(mapp, s, depth)
    
    s[0]-=1

def draw(mapp, p1, p2):
    for i in range(p1[1], p2[1]+1):
        for j in range(p1[0], p2[0]+1):
            mapp[(i,j)] = "#"

def part2(file: list[str]):
    mapp = dict()
    depth = 0
    for line in file:
        points = [[int(x) for x in ps.split(",")] for ps in line.split(" -> ")]

        for i in range(1, len(points)):
            p1, p2 = sorted(points[i-1:i+1])
            depth = max(p2[1], depth)
            draw(mapp, p1, p2)
    
    depth += 2
    r = 500 + (depth+1 // 2)
    l = 500 - (depth+1 // 2)

    draw(mapp, (l, depth), (r, depth))
    
    i, s = 0, [0, 500]
    while not sandFall(mapp, s, depth):
        mapp[(s[0],s[1])] = "o"
        i, s = i+1, [0, 500]
    
    print(i)