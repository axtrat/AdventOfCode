def visible(campo: list[list[int]], i:int, j: int) -> bool:
    ft, fb, fl, fr = True, True, True, True

    tree = campo[i][j]
    for k in range(i+1, len(campo)):
        if not (ft := (campo[k][j] < tree)):
            break
    
    for k in range(j+1, len(campo[i])):
        if not (fr := (campo[i][k] < tree)):
            break
        
    for k in reversed(range(i)):
        if not (fb := (campo[k][j] < tree)):
            break

    for k in reversed(range(j)):
        if not (fl := (campo[i][k] < tree)):
            break

    return ft or fb or fl or fr


def visibles(campo: list[list[int]]):
    res = list[int]()
    for i in range(len(campo)):
        for j in range(len(campo[i])):
            if visible(campo, i, j):
                res.append(campo[i][j])
                
    return res


def part1(file: list[str]):
    campo = list[list[int]]()
    for line in file:
        row = list(map(lambda c: int(c), line))
        campo.append(row)

    res = visibles(campo)

    print(len(res))