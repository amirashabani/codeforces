n = int(input())
groups = 1
previous = input()

for i in range(n-1):
    magnet = input()
    if magnet != previous:
        groups += 1

    previous = magnet

print(groups)
