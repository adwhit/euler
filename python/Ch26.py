def longdiv(n,m,amount):
    dig = [n/m]
    while len(dig) <= amount:
        n = 10*(n%m)
        dig.append(n/m)
    while dig[-1] == 0:
        del dig[-1]
    while dig[0] == 0:
        del dig[0]
    return dig

def getrecur(l):
    while l:
        t1=len(l)
        for pos in range(t1/2):
            recstr = l[:pos+1]
            t2 = len(recstr)
            teststr = recstr*(t1/t2) + recstr[:t1%t2]
            if l == teststr and not t1 == t2:
                return t2 
        del l[0]
    return 0

def doit(tomax,testlen):
    d = {}
    for x in range(2,tomax+2):
        r = getrecur(longdiv(1,x,testlen))
        d[x] = r
    return d

