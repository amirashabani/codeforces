# http://codeforces.com/problemset/problem/1/A

import math

n, m, a = map(int, input().split(" "))

result = math.ceil(n / a) * math.ceil(m /a)

print(result)