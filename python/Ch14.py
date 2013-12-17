
def nextn(n):
    if n%2 == 0:
        return n/2
    else:
        return 3*n+1


def chain(i):
    l=1
    while True:
        i = nextn(i)
        l = l+1
        if i == 1:
            break
    return l

def doit(r):
    big = 0
    for x in xrange(1,r):
        res = chain(x)
        if res > big:
            print x,res
            big = res 

