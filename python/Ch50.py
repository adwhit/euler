import helper as hp

p = hp.primes(1000000)
r = []

def doit():
    rund = 1
    for i in range(len(p)):
        r = p[i:i+rund]
        rs = sum(r)
        print r, rs
        if rs > 1000000:
            return best,rund
        while True:
            if rs > 1000000:
                break
            if rs in p:
                rund = len(r)
                best = rs
            nextp = p[i+len(r)]
            r.append(nextp)
            rs += nextp 

    


            



