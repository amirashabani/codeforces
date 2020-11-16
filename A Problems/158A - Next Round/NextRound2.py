# http://codeforces.com/problemset/problem/158/A

n, k = map(int, input().split(" "))
scores = list(map(int, input().split(" ")))
    
score = scores[k-1]
    
count = 0
    
for i in range(n):
    if scores[i] >= score and scores[i] > 0:
        count += 1
    
print(count)