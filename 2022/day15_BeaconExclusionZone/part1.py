import re

def distance(p1, p2):
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])

def inside(points, sensor, beacon, y):
    remain = distance(sensor, beacon) - abs(sensor[1] - y)
    if remain < 0:
        return

    points[beacon] = "B" 
    points[sensor] = "S" 
    
    for x in range(sensor[0]-remain, sensor[0]+remain+1):
        if (x, y) not in points:
            points[(x, y)] = "#" 
    
    

def printLine(points, y):
    minx = min(p[0] for p in points)
    maxx = max(p[0] for p in points)

    for x in range(minx, maxx+1):
        if (x,y) in points:
            print(points[(x, y)], end="")
        else:
            print(end=".")
    print()


def part1(file: list[str]):
    points = dict()

    y = 2000000 
    for line in file:
        line = [int(x) for x in re.split('Sensor at x=|, y=|: closest beacon is at x=|, y=', line)[1:]]
        
        inside(points, (line[0], line[1]), (line[2], line[3]), y)

    count = 0
    for point in points:
        if points[point] == "#":
            count+=1
    print(count)