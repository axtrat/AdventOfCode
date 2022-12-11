def score(a: int, x: int) -> int:
    if x == (a+1) % 3:
        score = 6 + x+1
    elif a == x:
        score = 3 + x+1
    else:
        score = x+1
    return score

def chose(a, x):
    match x:
        case "Z": return (a+1) % 3
        case "Y": return a
        case "X": return (a-1) % 3

def part2(file):
    sum = 0
    for line in file:
        ac, xc = line.split(" ")
        a = ord(ac)-ord("A")
        x = chose(a, xc)
        sum += score(a, x)

    print(sum)