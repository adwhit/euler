
def ispent(p):
    root = int(sqrt(1+24*p))
    if root**2 == 1+24*p: #root is integral
        if (root +1)%6 == 0:
            return True
    return False


def istri(p):
    root = int(sqrt(1+8*p))
    if root**2 == 1+8*p: #root is integral
        if (root +1)%2 == 0:
            return True
    return False

def genhex():
    n = 1
    while True:
        yield n*(2*n-1)
        n = n + 1

def doit():
    r = []
    g = genhex()
    while True:
        n = g.next()
        if ispent(n) and n > 40755:
            print n
            break
