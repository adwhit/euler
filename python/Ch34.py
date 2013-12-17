import scipy as sp

r = []
for x in range(3,200000):
    l = [int(i) for i in str(x)]
    if sum(sp.factorial(l)) == x:
        r.append(x)

print r

