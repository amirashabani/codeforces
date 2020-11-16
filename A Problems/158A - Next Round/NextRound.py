# http://codeforces.com/problemset/problem/158/A

n, k = map(int, input().split(" "))
scores = list(map(int, input().split(" ")))

advances = k
kth = scores[k-1]

for i in range(k, n):
    ith = scores[i]
    if ith >= kth and ith > 0:
        advances += 1

advances -= scores[:k].count(0)

print(advances)