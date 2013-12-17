import helper as hp
n = 1000000
p = set(hp.primes(n))
evens = set(range(2,n,2))
alln = set(range(2,n))
squares = [2*x**2 for x in range(1,n)]
comp = sorted(alln-(p|evens))

def doit():
    for x in comp:
        cf = True
        for y in squares:
            if y>x:
                break
            if x-y in p:
                cf = False
        if cf:
            return x


