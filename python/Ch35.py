#Slow!
import helper as hp

def rot(l):
    l2 = l[1:] +[l[0]]
    while True:
        yield int(''.join(map(str,l2))) 
        l2 = l2[1:] +[l2[0]]
        if l2 == l:
            break

def doit(n):
    p = list(hp.prime_gen(n))
    print "Got primes"
    r = []

    for x in p:
        pf = True
        l = [int(i) for i in str(x)]
        if any([(k%2==0 or k%5==0) for k in l]):
            pf = False
            continue
        for y in rot(l):
            if not y in p:
                pf = False
                break
        if pf:
            print x
            r.append(x)
    return len(r) + 2

        


