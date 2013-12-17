import csv
f = open('names.txt')

names = sorted(csv.reader(f).next())
d = {}

for ix,name in enumerate(names):
    t = sum([ord(letter) - 64 for letter in name])
    d[name] = t*(ix+1)

tot = 0

for key in d:
    tot += d[key]

print tot
