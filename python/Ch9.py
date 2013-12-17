import math

def issquare(n):
    if math.sqrt(n) == int(math.sqrt(n)):
        return int(math.sqrt(n))
    else:
        return False

for a in range(1000):
    for b in range(1000):
        c = issquare(a**2 + b**2)
        if c:
            if a+b+c == 1000:
                print a,b,c
