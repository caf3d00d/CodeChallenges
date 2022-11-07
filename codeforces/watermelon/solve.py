import sys

input = int(sys.stdin.read(3))

if input > 3 and input % 2 == 0:
    print("YES")
else:
    print("NO")
