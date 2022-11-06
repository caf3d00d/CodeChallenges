ns = int(input())

matrix = []
count = 0

for i in range(ns):
    k = 0
    matrix.append([int(j) for j in input().split()])
    for x in matrix[i]:
        if x == 1:
            k += 1
    if k >= 2:
        count += 1

print(count)
