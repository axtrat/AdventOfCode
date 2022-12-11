def score(campo: list[list[int]], i:int, j: int) -> bool:
    ft, fb, fl, fr = 0, 0, 0, 0

    tree = campo[i][j]
    for k in range(i+1, len(campo)):
        ft+=1
        if campo[k][j] >= tree:
            break
    
    for k in range(j+1, len(campo[i])):
        fr+=1
        if campo[i][k] >= tree:
            break
        
    for k in reversed(range(i)):
        fb+=1
        if campo[k][j] >= tree:
            break

    for k in reversed(range(j)):
        fl+=1
        if campo[i][k] >= tree:
            break

    return ft * fb * fl * fr


def maxVisibility(campo: list[list[int]]):
    max = 0
    for i in range(len(campo)):
        for j in range(len(campo[i])):
            s = score(campo, i, j)

            if s > max:
                max = s
                
    return max

def part2(file: list[str]):
    campo = list[list[int]]()
    for line in file:
        row = list(map(lambda c: int(c), line))
        campo.append(row)

    res = maxVisibility(campo)

    print(res)