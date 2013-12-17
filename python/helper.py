'''This is to store useful functions that need often'''

import math
import numpy as np

def prime_gen(tomax):
    r = np.arange(2,tomax+1)
    while any(r):
        p = r[0]
        yield p
        r = r[r%r[0] != 0]

def is_prime(n,p):
    #broken for n = 0,1
    if any(n%p == 0):
        return False 
    else:
        return True

def prime_factors(n):
    f = None
    for m in prime_gen(int(math.sqrt(n))):
        if n%m == 0:
            f = n/m
            break
    if f:
        return [m]+prime_factors(f)
    else:
        return [n] 

def divisors(n):
    dar = np.arange(1,n/2+1)
    dar = dar[n%dar==0]
    return sum(dar)

def primes(n):
    s=range(0,n+1)
    s[1]=0
    bottom=2
    top=n//bottom
    while (bottom*bottom<=n):
            while (bottom<=top):
                    if s[top]:
                            s[top*bottom]=0
                    top-=1
            bottom+=1
            top=n//bottom
    return [x for x in s if x]

def permute(l,lowtohigh=True):
    while not sorted(l,reverse=lowtohigh) == l: 
        for i in range(len(l)):
            l1 = l[i:]
            if sorted(l1,reverse=lowtohigh) != l1:
                continue
            tmp = l[i-1]
            if lowtohigh:
                ix = snl(tmp,l1)
            else:
                ix = lns(tmp,l1)
            l[i-1] = l1[ix]
            l[ix+i] = tmp
            l[i:] = sorted(l[i:],reverse=not lowtohigh)
            yield int(''.join(map(str,l)))
            break

def lns(n,l):
    t = [n-x if x < n else 10 for x in l]
    return t.index(min(t))

def snl(n,l):
    t = [x-n if x > n else 10 for x in l]
    return t.index(min(t))
