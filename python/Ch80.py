from decimal import *

getcontext().prec = 101

def ssum(x):
    return sum([int(x) for (i,x) in enumerate(str(Decimal(x).sqrt())) if i != 1 and i < 101])

sqrs = [i*i for i in range(10)]

print sum([ssum(x) for x in range(100) if not x in sqrs])
