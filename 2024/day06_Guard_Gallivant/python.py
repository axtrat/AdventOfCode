#Apre corretti.txt e loops.txt
import re


with open('corretti.txt', 'r') as f:
    corretti = [line.strip() for line in f.readlines()][0][1:-1]

with open('loops.txt', 'r') as f:
    loops = [line.strip() for line in f.readlines()][0][1:-1]

pattern = re.compile(r'\((\d+, \d+)\)')

corretti = set(re.findall(pattern, corretti))
loops = re.findall(pattern, loops)

cont = 0
for item in loops:
    if item in corretti:
        cont += 1
    else:
        print(item)

print(cont)
