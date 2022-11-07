nk = input().split()
p = input().split()

k_th = p[int(nk[1]) - 1]

count = 0
for i in p:
    if int(i) >= int(k_th) and int(i) > 0:
        count += 1


print(count)


