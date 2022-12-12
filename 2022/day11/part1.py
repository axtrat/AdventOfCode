class Monkey:
    def __init__(self, lines: list[str]):
        self.__items = [int(x) for x in lines[1].removeprefix("  Starting items: ").split(", ")]
        match lines[2].removeprefix("  Operation: new = old ").split():
            case ['+', 'old']:
                self.__op = lambda x: x + x
            case ['+', val]:
                self.__op = lambda x, y=int(val): x + y
            case ['*', 'old']:
                self.__op = lambda x: x * x
            case ['*', val]:
                self.__op = lambda x, y=int(val): x * y

        d  = int(lines[3].removeprefix("  Test: divisible by "))
        m1 = int(lines[4].removeprefix("    If true: throw to monkey "))
        m2 = int(lines[5].removeprefix("    If false: throw to monkey "))

        self.__test = lambda i: m1 if i%d == 0 else m2

    def hasItem(self) -> bool:
        return len(self.__items) > 0
    
    def addItem(self, item):
        self.__items.append(item)

    def play(self) -> tuple[int, int]:
        item = self.__items.pop(0)
        item = self.__op(item)
        item //= 3

        return self.__test(item), item


def round(ms: list[Monkey], count: list[int]):
	for i in range(len(ms)):
		while ms[i].hasItem():
			count[i] += 1
			dest, item = ms[i].play()
			ms[dest].addItem(item)


def part1(file: list[str]):
    ms = list[Monkey]()
    for i in range(0, len(file), 7):
        ms.append(Monkey(file[i:i+6]))

    count = [0 for i in range(len(ms))]

    for i in range(20):
        round(ms, count)


    max=[0, 0]
    for c in count:
        if c > max[0]:
            max[1]=max[0]
            max[0]=c
        elif c > max[1]:
            max[1]=c



    print(max[0] * max[1])
