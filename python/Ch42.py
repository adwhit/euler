def gentri(n):
    tri = []
    for x in range(1,n+1):
        tri.append(x*(x+1)/2)
    return tri

tri = gentri(30)
ct = 0

f = open('words.txt')
s = f.read()
s = s.replace('"','')
l = s.split(',')

for word in l:
    score = 0
    for c in word:
        score += ord(c) - 64
    if score in tri:
        print word, score
        ct += 1


