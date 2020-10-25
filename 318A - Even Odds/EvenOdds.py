import math

n, k = map(int, input().split())
half = math.ceil(n / 2)

if k <= half:
    print((2 * k) - 1)
else:
    print(2 * (k - half))
