import re

class Sensor:
    def __init__(self, sensor, beacon) -> None:
        self.__sensor = sensor
        self.beacon = beacon
        self.radius = self.distance(beacon)

    def distance(self, p1):
        return abs(self.__sensor[0] - p1[0]) + abs(self.__sensor[1] - p1[1])

    def isCloser(self, p1) -> bool:
        return self.distance(p1) <= self.radius

def part1(file: list[str]):
    sensors = list[Sensor]()
    minx, maxx = 2**16, 0

    for line in file:
        line = [int(x) for x in re.split('Sensor at x=|, y=|: closest beacon is at x=|, y=', line)[1:]]
        sensor = Sensor((line[0], line[1]), (line[2], line[3]))
        sensors.append(sensor)
        maxx = max(maxx, line[0]+sensor.radius, line[2])
        minx = min(minx, line[0]-sensor.radius, line[2])

    count, y = 0, 10#2000000

    for x in range(minx, maxx+1):
        if (x, y) == sensor or (x, y) == sensor.beacon:
            continue
        for sensor in sensors:
            if sensor.isCloser((x, y)):
                count+=1
                break
    
    print("l:",maxx+1-minx)  
    print(count)