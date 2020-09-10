# http://codeforces.com/problemset/problem/112/A

first = input().lower()
second = input().lower()
comparison = 0

if first < second:
    comparison = -1
elif first > second:
    comparison = 1

print(comparison)