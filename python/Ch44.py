from math import sqrt

def genpentag():
    n = 1
    while True:
        yield n*(3*n-1)/2
        n = n + 1


def doit():
    r = []
    l = []
    g = genpentag()
    while len(r) < 1:
        c = g.next()
        for x in l:
            if c-x in l:
                if ispent(c+x):
                    r.append([c,x,c-x,c+x])
                    return r
        l.append(c)
    return r

def ispent(p):
    root = int(sqrt(1+24*p))
    if root**2 == 1+24*p: #root is integral
        if (root +1)%6 == 0:
            return True
    return False


