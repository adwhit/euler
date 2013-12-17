import helper as hp
import numpy as np

def isabundant(n):
    if hp.divisors(n) > n:
        return True
    else:
        return False

def abundantlist(tomax):
    ar = np.arange(tomax)
    return ar[np.array(map(isabundant,ar))]

def abunsum(tomax):
    a = abundantlist(tomax)
    return np.add.outer(a,a)

def notabsum(tomax):
    a = abunsum(tomax).flatten()
    b = np.unique(a)
    t = np.arange(tomax)
    return np.setdiff1d(t,b)


