a = int(input())
b = int(input())
c = int(input())

expressions = []

expressions.append(a+b+c)
expressions.append(a*b*c)
expressions.append((a+b)*c)
expressions.append((a*b)+c)
expressions.append(a*(b+c))
expressions.append(a+(b*c))

print(max(expressions))