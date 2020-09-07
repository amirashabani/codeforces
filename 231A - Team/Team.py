# http://codeforces.com/problemset/problem/231/A

n = int(input())

count = 0
for i in range(n):
    if sum(map(int, input().split(" "))) >= 2:
        count += 1

print(count)
