from collections import defaultdict
from math import sqrt

def gentrip(n):
    d = defaultdict(list)
    for a in range(1,n):
        for b in range(a,n):
            if a + b > n:
                break
            c = int(sqrt(a**2 + b**2))
            if (a + b + c > n) or (not istrip(a,b,c)):
                continue
            d[a+b+c].append([a,b,c])
    return d
            

def istrip(a,b,c):
    if c**2 == a**2 + b**2:
        return True
    else:
        return False

