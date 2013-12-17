import helper as hp
from math import sqrt

def ispan(n):
    l = [int(s) for s in str(n)]
    g = set(l)
    if set(l) == set(range(1,len(str(n))+1)):
        return True
    else:
        return False

l = [9,8,7,6,5,4,3,2,1]
p = hp.primes(10000000)
r = []

def permute(l):
    while not sorted(l) == l: 
        for i in range(len(l)):
            l1 = l[i:]
            if sorted(l1) != l1:
                continue
            tmp = l[i-1]
            ix = lns(tmp,l1)
            l[i-1] = l1[ix]
            l[ix+i] = tmp
            l[i:] = sorted(l[i:],reverse=True)
            yield int(''.join(map(str,l)))
            break


def lns(n,l):
    t = [n-x if x < n else 10 for x in l]
    return t.index(min(t))

def doit():
    perm = permute(l)
    for x in perm:
        pf = True
        for pr in p:
            if x % pr == 0:
                pf = False
                break
        if pf == True:
            return x

def doit2():
    for x in p:
        if ispan(x):
            print x
            r.append(x)




