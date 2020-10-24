n = int(input())
feelings = []

for i in range(n):
    if i%2 == 0:
        feelings.append("I hate")
    else:
        feelings.append("I love")

result = " that ".join(feelings)
print(f"{result} it")