# http://codeforces.com/problemset/problem/263/A

one_row = 0
one_column = 0

for i in range(1, 6):
    row = input().replace(" ", "")
    if "1" in row:
        one_row = i
        one_column = row.index("1") + 1

minimum_moves = abs(one_row - 3) + abs(one_column - 3)
print(minimum_moves)