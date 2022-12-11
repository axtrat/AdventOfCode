def score(a: int, x: int) -> int:
    if x == (a+1) % 3:
        score = 6 + x+1
    elif a == x:
        score = 3 + x+1
    else:
        score = x+1
    return score

def part1(file):
    sum = 0
    for line in file:
        a, x = line.split(" ")
        ai, xi = ord(a)-ord("A"), ord(x)-ord("X")
        sum += score(ai, xi)

    print(sum)