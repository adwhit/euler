import numpy as np

def divisors(n):
    dar = np.arange(1,n/2+1)
    dar = dar[n%dar==0]
    return sum(dar)

def divlist(tomax):
    divdict = {}
    for x in range(1,tomax+1):
        divdict[x] = divisors(x)
    return divdict

def amlist(tomax):
    d = divlist(tomax)
    l = []
    for key in d:
        try:
            if key == d[d[key]]:
                if key != d[key]:
                    l.append([key,d[key]])
        except KeyError:
            continue
    return l





