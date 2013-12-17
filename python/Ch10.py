import numpy as np



def genprimes(tomax):
    r = np.arange(2,tomax)
    p = np.zeros(200000).astype(int)
    lp = 0
    while any(r):
        p[lp] = r[0]
        if lp%100 == 0:
            print p[lp]
        lp += 1
        r = r[r%r[0] != 0]

    return p[:lp]


