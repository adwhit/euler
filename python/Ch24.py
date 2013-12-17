import math
import itertools


f = math.factorial

def ff(x,n):
    n=n-1
    for c in itertools.count():
        if c*f(x-1) > n:
            return [c-1]+ff(x-1,n-((c-1)*f(x-1))) 
        elif c*f(x-1) == n:
            return [c]+[0]*(x-1)

def proof(x):
    r = []
    l = x+[0]
    for ix,x in enumerate(l):
        r.append(x*f(9-ix))
    print sum(s)
    
def work(l):
    n = [0,1,2,3,4,5,6,7,8,9]
    print l
    for x in l:
        print n.pop(x)

