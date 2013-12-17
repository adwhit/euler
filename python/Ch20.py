import numpy as np
import math
import helper as hp
from operator import mul

def fact(n):
    ar = range(1,n+1)
    return reduce(mul,ar) 

def factfactors(n):
    f = []
    for x in range(1,101):
        f.extend(hp.prime_factors(x))
    return f

def rm25pair(l):
    while True:
        if 2 in l:
            if 5 in l:
                l.remove(2)
                l.remove(5)
                continue
        break
    return l


