class CPU:
    def __init__(self):
        self.__clock = 1
        self.__register = 1
        self.__sum = 0

    def cicle(self, val: int):
        if (self.__clock-20) % 40 == 0:
            self.__sum += (self.__clock * self.__register)

        self.__clock += 1
        self.__register += val

    def operate(self, operation: str):
        match operation.split():
            case ["addx", val]:
                self.cicle(0)
                self.cicle(int(val))
            case ["noop"]:
                self.cicle(0)

    def __str__(self) -> str:
        return str(self.__sum)


def part1(file: list[str]):
    c = CPU()
    for line in file:
        c.operate(line)

    print(c)
