r = []

def fifthp(n):
    return sum([int(x)**5 for x in list(str(n))])

for x in range(2,1000000):
    if fifthp(x) == x:
        r.append(x)
