n = int(input())
is_easy = True
line = input().split()

for number in line:
    if number == '1':
        is_easy = False

if is_easy:
    print("EASY")
else:
    print("HARD")
