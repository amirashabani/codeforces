# http://codeforces.com/problemset/problem/118/A

vowels = "aoyeui"

line = input().lower()

no_vowels = line.translate({ord(c): None for c in vowels})
dotted = '.'.join(list(no_vowels))
print(f".{dotted}")