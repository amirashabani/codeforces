# http://codeforces.com/problemset/problem/282/A

n = int(input())
x = 0

for i in range(n):
    line = input()
    if "+" in line:
        x += 1
    elif "-" in line:
        x -= 1

print(x)