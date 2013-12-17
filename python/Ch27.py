import helper as hp
import numpy as np

def gq(a,b,n):
    return n**2 + a*n + b

def parray():
    return np.array(list(hp.prime_gen(gq(1000,1000,guessmax))))

def doit(val,p):
    a = np.arange(-val + 1,val)
    b = p[p<val]
    t = np.ones([len(a),len(b)]).astype(bool)
    n=0
    while t.any():
        print n
        told = t.copy()
        l = np.add.outer(n**2 +a*n,b)
        logic = np.zeros([len(a),len(b)]).astype(bool)
        #print l
        for pr in p[np.logical_and(np.min(l)<=p, p<=np.max(l))]:
            l1 = (l==pr)
            logic = np.logical_or(logic,l1)
        t = np.logical_and(t,logic)
        #print logic 
        n = n + 1
    print n-1
    print a[np.nonzero(told)[0]]
    print b[np.nonzero(told)[1]]
    return told

def printall(n,a,b):
    for x in range(n+1):
        print gq(a,b,x)
        print hp.is_prime(gq(a,b,x))
