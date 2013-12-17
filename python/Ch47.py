import helper as hp
pf = hp.prime_factors
n=646

while True:
    if len(set(pf(n))) == 4:
        if len(set(pf(n+1))) == 4:
            if len(set(pf(n+2))) == 4:
                if len(set(pf(n+3))) == 4:
                    print "Winner!",n
                    break
    n += 1
