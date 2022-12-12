class CTR:
    def __init__(self):
        self.__clock = 1
        self.__register = 1
        self.__row = ""

    def cicle(self, val: int):
        cul = (self.__clock) % 40
        if cul in range(self.__register, self.__register+3):
            self.__row += "#"
        else:
            self.__row += " "
        
        if cul == 0:
            self.__row += "\n"

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
        return str(self.__row)


def part2(file: list[str]):
    c = CTR()
    for line in file:
        c.operate(line)

    print(c)