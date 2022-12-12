class Monkey:
    def __init__(self, lines: list[str]):
        self.__items = [int(x) for x in lines[1].removeprefix(
            "  Starting items: ").split(", ")]
        match lines[2].removeprefix("  Operation: new = old ").split():
            case ['+', 'old']:
                self.__op = lambda x: x + x
            case ['+', val]:
                self.__op = lambda x, y=int(val): x + y
            case ['*', 'old']:
                self.__op = lambda x: x * x
            case ['*', val]:
                self.__op = lambda x, y=int(val): x * y

        self.d = int(lines[3].removeprefix("  Test: divisible by "))
        m1 = int(lines[4].removeprefix("    If true: throw to monkey "))
        m2 = int(lines[5].removeprefix("    If false: throw to monkey "))

        self.__test = lambda i: m1 if i % self.d == 0 else m2

    def hasItem(self) -> bool:
        return len(self.__items) > 0

    def addItem(self, item):
        self.__items.append(item)

    def play(self) -> tuple[int, int]:
        item = self.__items.pop(0)
        item = self.__op(item)

        return self.__test(item), item

class Game:
    def __init__(self, file: list[str]) -> None:
        self.__monkeys = list[Monkey]()
        self.__count = list[int]()
        self.__MCM = 1
        
        for i in range(0, len(file), 7):
            m = Monkey(file[i:i+6])
            self.__MCM *= m.d
            self.__count.append(0)
            self.__monkeys.append(m)

    def round(self):
        for i in range(len(self.__monkeys)):
            while self.__monkeys[i].hasItem():
                self.__count[i] += 1
                dest, item = self.__monkeys[i].play()
                self.__monkeys[dest].addItem(item % self.__MCM)

    def play(self, n: int):
        for i in range(n):
            self.round()

    def monkeyBisnesLevel(self) -> int:
        max = [0, 0]
        for c in self.__count:
            if c > max[0]:
                max[1] = max[0]
                max[0] = c
            elif c > max[1]:
                max[1] = c

        return max[0] * max[1]


def part2(file: list[str]):
    g = Game(file)

    g.play(10000)

    print(g.monkeyBisnesLevel())
