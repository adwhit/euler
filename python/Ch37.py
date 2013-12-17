import helper as hp

def truncl(n):
    while n:
        try:
            n= int(str(n)[1:])
            yield n
        except ValueError:
            n = None

def truncr(n):
    while n:
        try:
            n= int(str(n)[:-1])
            yield n
        except ValueError:
            n = None

def doit(n):
    evn = []
    r = []
    p = list(hp.prime_gen(n))
    print "Got Primes"
    for x in p:
        pf = True
        s = str(x)
        for ev in evn:
            if ev in s:
                pf = False
                break
        if pf:
            for tr in truncr(x):
                if tr not in p:
                    pf = False
                    break
        if pf:
            for tl in truncl(x):
                if tl not in p:
                    pf = False
                    break
        if pf:
            r.append(x)
            print r
    return [y for y in r if y > 10]

doit(20000)
