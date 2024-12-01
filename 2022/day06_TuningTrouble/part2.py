def allDifferent(string: str) -> bool:
    for i in range(len(string)-1):
        if string[i] in string[i+1:]:
            return False
    return True 
    
def part2(file: list[str]):
    stream = file[0]
    for i in range(14, len(stream)):
        buffer = stream[i-14:i]

        if allDifferent(buffer):
            print(i)
            break